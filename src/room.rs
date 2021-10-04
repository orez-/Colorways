use piston_window::{Context, DrawState, Image};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use geo::polygon;
use crate::color::Color;
use crate::entity::{Block, Entity, Exit, Lightbulb, LightSwitch, Player, Water};
use crate::line_of_sight::{line_of_sight, Visibility};

const ONE_START_MSG: &str = "level must have exactly one starting position";
pub const NUM_LEVELS: usize = 5;
const LEVELS: [&[u8]; NUM_LEVELS] = [
    include_bytes!("../bin/levels/level1.skb"),
    include_bytes!("../bin/levels/level2.skb"),
    include_bytes!("../bin/levels/level3.skb"),
    include_bytes!("../bin/levels/level4.skb"),
    include_bytes!("../bin/levels/level5.skb"),
];
const TITLE_LEVEL: &[u8] = include_bytes!("../bin/levels/title.skb");
const TILE_SIZE: f64 = 16.;
const WALL: [f64; 4] = [32., 0., TILE_SIZE, TILE_SIZE];
const FLOOR: [f64; 4] = [32., 16., TILE_SIZE, TILE_SIZE];

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
}
use Tile::*;

impl Tile {
    fn from_chr(chr: char) -> Self {
        match chr {
            '#' => Wall,
            _ => Floor,
        }
    }

    pub fn sprite(&self, x: usize, y: usize) -> Image {
        let src = match self {
            Wall => WALL,
            Floor => FLOOR,
        };
        Image::new()
            .src_rect(src)
            .rect([x as f64 * TILE_SIZE, y as f64 * TILE_SIZE, TILE_SIZE, TILE_SIZE])
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Wall => false,
            Floor => true,
        }
    }

    pub fn is_transparent(&self) -> bool {
        // TODO: ᖍ(∙⟞∙)ᖌ
        self.is_passable()
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

type Game = (Room, Player, Vec<Entity>, Color);

pub struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    sees_color: Vec<[bool; 3]>
}

impl Room {
    pub fn new(level: usize) -> Game {
        Room::from_file(LEVELS[level])
    }

    pub fn new_title() -> Game {
        Room::from_file(TITLE_LEVEL)
    }

    pub fn from_file(bytes: &[u8]) -> Game {
        let starting_color = match bytes[0] {
            b'R' => Color::Red,
            b'G' => Color::Green,
            b'B' => Color::Blue,
            _ => Color::Gray,
        };
        let first_line = bytes.iter().position(|&c| c == b'\n').unwrap() + 1;
        let bytes = &bytes[first_line..];
        let width = bytes.iter().position(|&c| c == b'\n').unwrap();
        let tiles: Vec<_> = bytes.iter()
            .filter_map(|&c| {
                (c != b'\n').then(|| Tile::from_chr(c as char))
            }).collect();
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
                'k' => { entities.push(Entity::Block(Block::new(x, y, Color::Gray))); },
                'r' => { entities.push(Entity::Block(Block::new(x, y, Color::Red))); },
                'g' => { entities.push(Entity::Block(Block::new(x, y, Color::Green))); },
                'b' => { entities.push(Entity::Block(Block::new(x, y, Color::Blue))); },
                'w' => { entities.push(Entity::Block(Block::new(x, y, Color::White))); },
                'R' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx][0] = true;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Red, polygon_pts)));
                },
                'G' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx][1] = true;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Green, polygon_pts)));
                },
                'B' => {
                    let Visibility { polygon_pts, tiles } = line_of_sight(x, y, width, height, &walls_polygon);
                    for idx in tiles {
                        sees_color[idx][2] = true;
                    }
                    entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Blue, polygon_pts)));
                },
                '1' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::Red))); },
                '2' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::Green))); },
                '3' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::Blue))); },
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

        (
            Room { width, height, tiles, sees_color },
            player.expect(ONE_START_MSG),
            entities,
            starting_color,
        )
    }

    pub fn render(&self,
                  texture: &GlTexture,
                  draw_state: &DrawState,
                  context: &Context,
                  gl: &mut GlGraphics) {
        for (i, elem) in self.tiles.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            elem.sprite(x, y)
                .draw(texture, draw_state, context.transform, gl);
        }
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 { return None; }
        let idx = self.width * (y as usize) + x as usize;
        self.tiles.get(idx).cloned()
    }

    pub fn tile_in_light(&self, x: i32, y: i32, color: &Color) -> bool {
        let cidx = match color {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
            _ => { return false; },
        };
        if x < 0 || y < 0 { return false; }
        let idx = self.width * (y as usize) + x as usize;
        self.sees_color.get(idx).map_or(false, |&arr| arr[cidx])
    }

    pub fn pixel_width(&self) -> i64 {
        self.width as i64 * 16
    }

    pub fn pixel_height(&self) -> i64 {
        self.height as i64 * 16
    }
}
