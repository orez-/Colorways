use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::color::Color;
use crate::view::{GameAction, GameView};

const TILE_SIZE: f64 = 16.;
const WATER: [f64; 4] = [32., 64., TILE_SIZE, TILE_SIZE];

pub struct Water {
    pub x: i32,
    pub y: i32,
}

impl Water {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new()
            .src_rect(WATER)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn is_approachable(&self, _direction: &Direction, _game: &GameView) -> Option<GameAction> {
        Some(GameAction::Stop)
    }
    pub fn on_approach(&mut self, _direction: &Direction) -> Option<GameAction> {
        None
    }
}
