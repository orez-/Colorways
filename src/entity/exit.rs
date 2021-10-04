use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::view::{GameAction, GameView};

const TILE_SIZE: f64 = 16.;
const EXIT: [f64; 4] = [16., 0., TILE_SIZE, TILE_SIZE];

pub struct Exit {
    pub x: i32,
    pub y: i32,
}

impl Exit {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new()
            .src_rect(EXIT)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }
    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn is_approachable(&self, _direction: &Direction, _game: &GameView) -> bool { true }
    pub fn on_approach(&mut self, _direction: &Direction) -> Option<GameAction> {
        Some(GameAction::Win)
    }
}
