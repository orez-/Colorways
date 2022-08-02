use piston_window::Image;
use crate::app::Direction;
use crate::color::Color;
use crate::entity::IEntity;
use crate::scene::{HeadlessScene, GameAction};

const TILE_SIZE: f64 = 16.;
const LIGHTSWITCH: [f64; 4] = [0., 48., TILE_SIZE, TILE_SIZE];

pub struct LightToggle {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl LightToggle {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }
}

impl IEntity for LightToggle {
    fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(LIGHTSWITCH)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    fn on_approach(&self, _entity_id: usize, _direction: Direction, scene: &HeadlessScene) -> GameAction {
        if scene.tile_in_light(self.x, self.y, self.color) { return GameAction::Walk; }
        GameAction::ColorToggle(self.color)
    }
}
