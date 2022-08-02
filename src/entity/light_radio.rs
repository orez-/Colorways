use piston_window::Image;
use crate::app::Direction;
use crate::color::Color;
use crate::entity::IEntity;
use crate::scene::{HeadlessScene, GameAction};

const TILE_SIZE: f64 = 16.;
const LIGHTSWITCH: [f64; 4] = [0., 32., TILE_SIZE, TILE_SIZE];

/// A light switch which enables the light of its color and disables
/// all other colors, like a radio button.
pub struct LightRadio {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl LightRadio {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }
}

impl IEntity for LightRadio {
    fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(LIGHTSWITCH)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    fn on_approach(&self, _entity_id: usize, _direction: Direction, _game: &HeadlessScene) -> GameAction {
        GameAction::ColorRadio(self.color)
    }
}
