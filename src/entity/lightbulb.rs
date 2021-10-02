use piston_window::{Image, UpdateArgs};
use crate::color::Color;
use crate::view::Direction;

const TILE_SIZE: f64 = 16.;
const LIGHTBULB: [f64; 4] = [16., 16., TILE_SIZE, TILE_SIZE];

pub struct Lightbulb {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl Lightbulb {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Lightbulb { x, y, color }
    }

    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(LIGHTBULB)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn push(&mut self, _direction: &Direction) { }
}
