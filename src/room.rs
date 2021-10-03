use std::collections::HashMap;
use bitmask_enum::bitmask;
use piston_window::{Context, DrawState, Image};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use geo::polygon;
use crate::entity::{Block, Entity, Lightbulb, LightSwitch, Player};
use crate::color::Color;

const ONE_START_MSG: &str = "level must have exactly one starting position";
const LEVEL2: &[u8] = include_bytes!("../bin/levels/level3.skb");
// const LEVEL2: &[u8] = include_bytes!("../bin/levels/playground.skb");
const TILE_SIZE: f64 = 16.;
const WALL: [f64; 4] = [32., 0., TILE_SIZE, TILE_SIZE];
const FLOOR: [f64; 4] = [32., 16., TILE_SIZE, TILE_SIZE];
const EXIT: [f64; 4] = [16., 0., TILE_SIZE, TILE_SIZE];

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
    Exit,
}
use Tile::*;

impl Tile {
    fn from_chr(chr: char) -> Self {
        match chr {
            '#' => Wall,
            'z' => Exit,
            _ => Floor,
        }
    }

    pub fn sprite(&self, x: usize, y: usize) -> Image {
        let src = match self {
            Wall => WALL,
            Floor => FLOOR,
            Exit => EXIT,
        };
        Image::new()
            .src_rect(src)
            .rect([x as f64 * TILE_SIZE, y as f64 * TILE_SIZE, TILE_SIZE, TILE_SIZE])
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Wall => false,
            Floor => true,
            Exit => true,
        }
    }

    pub fn is_transparent(&self) -> bool {
        // TODO: ᖍ(∙⟞∙)ᖌ
        self.is_passable()
    }
}

// square is on the right_ of the line
// ^->
// |#|
// <-v
#[bitmask(u8)]
enum SquareEdge {
    Up,
    Right,
    Down,
    Left,
}

fn pop<K, V>(map: &mut HashMap<K, V>) -> Option<(K, V)>
where K: Eq + std::hash::Hash + Clone,
{
    let k = map.keys().next()?.clone();
    map.remove_entry(&k)
}

fn to_linestring(cs: Vec<(usize, usize)>) -> geo::LineString<f64> {
    geo::LineString::from(cs.into_iter().map(|(x, y)| [x as f64 * TILE_SIZE, y as f64 * TILE_SIZE]).collect::<Vec<_>>())
}

fn to_polygon(exterior: Vec<(usize, usize)>, interior: Vec<Vec<(usize, usize)>>) -> geo::Polygon<f64> {
    let exterior = to_linestring(exterior);
    let interior = interior.into_iter().map(to_linestring).collect();
    geo::Polygon::new(exterior, interior)
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
    pub walls_polygon: geo::MultiPolygon<f64>,
}

impl Room {
    pub fn new() -> Game {
        Room::from_file(LEVEL2)
    }

    pub fn from_file(bytes: &[u8]) -> Game {
        let width = bytes.iter().position(|&c| c == b'\n').unwrap();
        let tiles: Vec<_> = bytes.iter()
            .filter_map(|&c| {
                (c != b'\n').then(|| Tile::from_chr(c as char))
            }).collect();
        let height = tiles.len() / width;

        let walls_polygon = to_walls_polygon(&tiles, width);
        let room = Room { width, height, tiles, walls_polygon };

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
                'R' => { entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Red, &room))); },
                'G' => { entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Green, &room))); },
                'B' => { entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Blue, &room))); },
                '1' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::Red))); },
                '2' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::Green))); },
                '3' => { entities.push(Entity::LightSwitch(LightSwitch::new(x, y, Color::Blue))); },
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
            room,
            player.expect(ONE_START_MSG),
            entities,
            Color::Green,
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

    pub fn pixel_width(&self) -> i64 {
        self.width as i64 * 16
    }

    pub fn pixel_height(&self) -> i64 {
        self.height as i64 * 16
    }
}
