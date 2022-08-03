use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::color::Color;
use crate::entity::IEntity;
use crate::entity::intensity::Intensity;
use crate::scene::{HeadlessScene, GameAction};

const TILE_SIZE: f64 = 16.;
const LIGHTSWITCH: [f64; 4] = [0., 32., TILE_SIZE, TILE_SIZE];

/// A light switch which enables the light of its color and disables
/// all other colors, like a radio button.
pub struct LightRadio {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    intensity: Intensity,
}

impl LightRadio {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color, intensity: Intensity::new(1.) }
    }

    pub fn set_in_light(&mut self, in_light: bool) {
        let intensity = if in_light { 0.5 }
            else { 1. };
        self.intensity.goal = intensity;
    }
}

impl IEntity for LightRadio {
    fn update(&mut self, args: &UpdateArgs) {
        self.intensity.update(args);
    }

    fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        let mut color = self.color.as_component();
        color[3] = self.intensity.val;
        Image::new_color(color)
            .src_rect(LIGHTSWITCH)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    fn on_approach(&self, _entity_id: usize, _direction: Direction, scene: &HeadlessScene) -> GameAction {
        if scene.tile_in_light(self.x, self.y, self.color) { return GameAction::Walk; }
        GameAction::ColorRadio(self.color)
    }
}
