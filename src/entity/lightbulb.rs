use piston_window::{Image, UpdateArgs};
use crate::color::Color;
use crate::room::Room;
use crate::line_of_sight::{Visibility, line_of_sight};
use crate::view::{Direction, GameView};

const TILE_SIZE: f64 = 16.;
const LIGHTBULB: [f64; 4] = [16., 16., TILE_SIZE, TILE_SIZE];

pub struct Lightbulb {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub light_polygon: Vec<[f64; 2]>,
}

impl Lightbulb {
    pub fn new(x: i32, y: i32, color: Color, room: &Room) -> Self {
        let Visibility { polygon_pts, .. } = line_of_sight(x, y, room);
        Self { x, y, color, light_polygon: polygon_pts }
    }

    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(LIGHTBULB)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn is_approachable(&self, _direction: &Direction, _game: &GameView) -> bool {
        false
    }
    pub fn on_approach(&mut self, _direction: &Direction) { }
}
