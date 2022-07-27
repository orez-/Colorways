use crate::color::Color;
use crate::entity::{Block, Entity, Exit, Lightbulb, LightSwitch, Player, Water};
use crate::line_of_sight::{line_of_sight, Visibility};
use crate::room::{Room, Tile};
use geo::polygon;

const ONE_START_MSG: &str = "level must have exactly one starting position";
pub const NUM_LEVELS: usize = 8;
const LEVELS: [&[u8]; NUM_LEVELS] = [
    include_bytes!("../bin/levels/level01.skb"),
    include_bytes!("../bin/levels/level02.skb"),
    include_bytes!("../bin/levels/level03.skb"),
    include_bytes!("../bin/levels/level04.skb"),
    include_bytes!("../bin/levels/level05.skb"),
    include_bytes!("../bin/levels/level06.skb"),
    include_bytes!("../bin/levels/level07.skb"),
    include_bytes!("../bin/levels/level08.skb"),
];
const TITLE_LEVEL: &[u8] = include_bytes!("../bin/levels/title.skb");
const TILE_SIZE: f64 = 16.;

pub struct SceneConfig {
    pub room: Room,
    pub player: Player,
    pub entities: Vec<Entity>,
    pub starting_color: Color,
}

impl SceneConfig {
    pub fn new(level: usize) -> Self {
        SceneConfig::from_file(LEVELS[level])
    }

    pub fn new_title() -> Self {
        SceneConfig::from_file(TITLE_LEVEL)
    }

    pub fn from_file(bytes: &[u8]) -> Self {
        let starting_color = match bytes[0] {
            b'R' => Color::RED,
            b'G' => Color::GREEN,
            b'B' => Color::BLUE,
            _ => Color::GRAY,
        };
        let first_line = bytes.iter().position(|&c| c == b'\n').unwrap() + 1;
        let bytes = &bytes[first_line..];
        let width = bytes.iter().position(|&c| c == b'\n').unwrap();
        let tiles: Vec<_> = bytes.iter()
            .filter_map(|&c| (c != b'\n').then(|| Tile::from_byte(c)))
            .collect();
        let height = tiles.len() / width;

        let walls_polygon = to_walls_polygon(&tiles, width);
        let mut sees_color = vec![[false, false, false]; tiles.len()];

        let mut x = 0;
        let mut y = 0;
        let mut player = None;
        let mut entities = Vec::new();
        for &byte in bytes {
            match byte as char {
                'a' => {
                    if player.is_some() { panic!("{}", ONE_START_MSG); }
                    player = Some(Player::new(x, y));
                },
                'k' => { entities.push(Entity::Block(Block::new(x, y, Color::GRAY))); },
                'r' => { entities.push(Entity::Block(Block::new(x, y, Color::RED))); },
                'g' => { entities.push(Entity::Block(Block::new(x, y, Color::GREEN))); },
                'b' => { entities.push(Entity::Block(Block::new(x, y, Color::BLUE))); },
                'y' => { entities.push(Entity::Block(Block::new(x, y, Color::YELLOW))); },
                'c' => { entities.push(Entity::Block(Block::new(x, y, Color::CYAN))); },
                'm' => { entities.push(Entity::Block(Block::new(x, y, Color::MAGENTA))); },
                'w' => { entities.push(Entity::Block(Block::new(x, y, Color::WHITE))); },
                'R' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx][0] = true;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::RED, polygon_pts)));
                },
                'G' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx][1] = true;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::GREEN, polygon_pts)));
                },
                'B' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx][2] = true;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::BLUE, polygon_pts)));
                },
                '1' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::RED))); },
                '2' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::GREEN))); },
                '3' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::BLUE))); },
                'z' => { entities.push(Entity::Exit(Exit::new(x, y))); },
                '~' => { entities.push(Entity::Water(Water::new(x, y))); },
                '\n' => {
                    x = 0;
                    y += 1;
                    continue;
                },
                _ => (),
            }
            x += 1;
        }

        SceneConfig {
            room: Room { width, height, tiles, sees_color },
            player: player.expect(ONE_START_MSG),
            entities,
            starting_color,
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
