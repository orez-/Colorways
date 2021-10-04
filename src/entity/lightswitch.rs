use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::color::Color;
use crate::view::{GameAction, GameView};

const TILE_SIZE: f64 = 16.;
const LIGHTSWITCH: [f64; 4] = [0., 32., TILE_SIZE, TILE_SIZE];

pub struct LightSwitch {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl LightSwitch {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }

    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(LIGHTSWITCH)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn is_approachable(&self, _direction: &Direction, _game: &GameView) -> bool {
        true
    }
    pub fn on_approach(&mut self, _direction: &Direction) -> Option<GameAction> {
        Some(GameAction::ColorChange(self.color.clone()))
    }
}
