use crate::color::Color;
use crate::entity::{Block, Entity, Exit, Lightbulb, LightRadio, LightToggle, Player, Water};
use crate::line_of_sight::{line_of_sight, Visibility};
use crate::room::{Room, Tile};
use crate::scene::{Camera, CameraMode, HeadlessScene, SceneTag};
use geo::polygon;

const AMBIENT_DEFAULT: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
const ONE_START_MSG: &str = "level must have exactly one starting position";
pub const NUM_LEVELS: usize = 9;
const LEVELS: [&[u8]; NUM_LEVELS] = [
    include_bytes!("../bin/levels/level01.skb"),
    include_bytes!("../bin/levels/level02.skb"),
    include_bytes!("../bin/levels/level03.skb"),
    include_bytes!("../bin/levels/level04.skb"),
    include_bytes!("../bin/levels/level05.skb"),
    include_bytes!("../bin/levels/level06.skb"),
    include_bytes!("../bin/levels/level07.skb"),
    include_bytes!("../bin/levels/level08.skb"),
    include_bytes!("../bin/levels/level09.skb"),
];
const TITLE_LEVEL: &[u8] = include_bytes!("../bin/levels/title.skb");
const TILE_SIZE: f64 = 16.;

// nightly-only as of 7/29/21 😩
fn take_first<'a, T>(slice: &mut &'a [T]) -> Option<&'a T> {
    let (first, rem) = slice.split_first()?;
    *slice = rem;
    Some(first)
}

fn take<'a, T>(slice: &mut &'a [T], range: std::ops::RangeTo<usize>) -> Option<&'a [T]> {
    if range.end > slice.len() {
        return None;
    }
    let (front, rem) = slice.split_at(range.end);
    *slice = rem;
    Some(front)
}

pub struct SceneConfig {
    pub state: HeadlessScene,
    pub ambient_color: [f32; 4],
    pub tag: Option<SceneTag>,
    pub camera: Camera,
}

impl SceneConfig {
    pub fn new(level: usize) -> Self {
        SceneConfig::from_file(LEVELS[level])
    }

    pub fn new_title() -> Self {
        let mut sc = SceneConfig::from_file(TITLE_LEVEL);
        sc.ambient_color = [0.9, 0.9, 0.9, 1.0];
        sc
    }

    pub fn from_file(bytes: &[u8]) -> Self {
        let starting_color = match bytes[0] {
            b'R' => Color::RED,
            b'G' => Color::GREEN,
            b'B' => Color::BLUE,
            _ => Color::GRAY,
        };
        let x_mode = match bytes[1] {
            b'p' => CameraMode::Player,
            b'+' => CameraMode::Centered,
            t => { panic!("Unrecognized camera mode {:?}", t as char); }
        };
        let y_mode = match bytes[2] {
            b'p' => CameraMode::Player,
            b'+' => CameraMode::Centered,
            t => { panic!("Unrecognized camera mode {:?}", t as char); }
        };
        let camera = Camera { x_mode, y_mode };
        let tag = match bytes[3] {
            b'm' => Some(SceneTag::TeachMove),
            b'u' => Some(SceneTag::TeachUndo),
            b'\n' => None,
            t => { panic!("Unrecognized tag id {:?}", t as char); }
        };
        let first_line = bytes.iter().position(|&c| c == b'\n').unwrap() + 1;
        let mut bytes = &bytes[first_line..];
        // XXX: we're not strictly ascii anymore!!
        // But... the top line should always be walls ### ..
        // So maybe this is okay ᖍ(シ)ᖌ
        let width = bytes.iter().position(|&c| c == b'\n').unwrap();
        let tiles: Vec<_> = bytes.iter()
            .filter_map(|&c| match c {
                b'#' => Some(Tile::Wall),
                b'\n' | b'\x80'..=b'\xff' => None,
                _ => Some(Tile::Floor),
            }).collect();
        let height = tiles.len() / width;

        let walls_polygon = to_walls_polygon(&tiles, width);
        let mut sees_color = vec![Color::GRAY; tiles.len()];

        let mut x = 0;
        let mut y = 0;
        let mut player = None;
        let mut entities = Vec::new();
        while let Some(byte) = take_first(&mut bytes) {
            match byte {
                b'a' => {
                    if player.is_some() { panic!("{ONE_START_MSG}"); }
                    player = Some(Player::new(x, y));
                }
                b'k' => { entities.push(Self::parse_entity_type(x, y, Color::GRAY, take(&mut bytes, ..2))); }
                b'r' => { entities.push(Self::parse_entity_type(x, y, Color::RED, take(&mut bytes, ..2))); }
                b'g' => { entities.push(Self::parse_entity_type(x, y, Color::GREEN, take(&mut bytes, ..2))); }
                b'b' => { entities.push(Self::parse_entity_type(x, y, Color::BLUE, take(&mut bytes, ..2))); }
                b'y' => { entities.push(Self::parse_entity_type(x, y, Color::YELLOW, take(&mut bytes, ..2))); }
                b'c' => { entities.push(Self::parse_entity_type(x, y, Color::CYAN, take(&mut bytes, ..2))); }
                b'm' => { entities.push(Self::parse_entity_type(x, y, Color::MAGENTA, take(&mut bytes, ..2))); }
                b'w' => { entities.push(Self::parse_entity_type(x, y, Color::WHITE, take(&mut bytes, ..2))); }
                b'R' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx] |= Color::RED;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::RED, polygon_pts)));
                }
                b'G' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx] |= Color::GREEN;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::GREEN, polygon_pts)));
                }
                b'B' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx] |= Color::BLUE;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::BLUE, polygon_pts)));
                }
                b'z' => { entities.push(Entity::Exit(Exit::new(x, y))); }
                b'~' => { entities.push(Entity::Water(Water::new(x, y))); }
                b'\n' => {
                    x = 0;
                    y += 1;
                    continue;
                }
                b'#' | b'.' => (),
                c => { panic!("Unexpected byte {c:#02x}"); }
            }
            x += 1;
        }

        SceneConfig {
            state: HeadlessScene::new(
                player.expect(ONE_START_MSG),
                Room { width, height, tiles, sees_color },
                entities,
                starting_color,
            ),
            ambient_color: AMBIENT_DEFAULT,
            tag,
            camera,
        }
    }

    fn parse_entity_type(x: i32, y: i32, color: Color, modifier: Option<&[u8]>) -> Entity {
        match modifier {
            Some(b"\xcc\x82") => Entity::LightToggle(LightToggle::new(x, y, color)),
            Some(b"\xcc\x8a") => Entity::LightRadio(LightRadio::new(x, y, color)),
            Some(b"\xcc\xbd") => Entity::Block(Block::new(x, y, color)),
            Some(c) => { panic!("Unexpected modifier bytes {c:x?}"); }
            None => { panic!("Not enough modifier bytes found"); }
        }
    }
}

fn to_walls_polygon(tiles: &[Tile], width: usize) -> geo::MultiPolygon<f64> {
    let mut polygons = Vec::new();
    for (i, tile) in tiles.iter().enumerate() {
        if !tile.is_transparent() {
            let x = (i % width) as f64 * TILE_SIZE;
            let y = (i / width) as f64 * TILE_SIZE;

            polygons.push(polygon![
                exterior: [
                    (x: x, y: y),
                    (x: x + TILE_SIZE, y: y),
                    (x: x + TILE_SIZE, y: y + TILE_SIZE),
                    (x: x, y: y + TILE_SIZE),
                ],
                interiors: [],
            ]);
        }
    }
    geo::MultiPolygon(polygons)
}
