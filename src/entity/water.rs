use piston_window::Image;
use crate::app::Direction;
use crate::color::Color;
use crate::entity::IEntity;
use crate::scene::{Scene, GameAction};

const TILE_SIZE: f64 = 16.;
const WATER: [f64; 4] = [32., 64., TILE_SIZE, TILE_SIZE];
const SPLASH: [f64; 4] = [48., 176., TILE_SIZE, TILE_SIZE];

enum State {
    Idle,
    Sinking(f64),
}

pub struct Water {
    pub x: i32,
    pub y: i32,
    state: State
}

impl Water {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, state: State::Idle }
    }
}

impl IEntity for Water {
    fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        let src = match self.state {
            State::Idle => WATER,
            State::Sinking(_) => SPLASH,
        };
        Image::new()
            .src_rect(src)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    fn on_approach(&self, _entity_id: usize, _direction: Direction, scene: &Scene) -> GameAction {
        if scene.tile_in_light(self.x, self.y, Color::BLUE) { return GameAction::Walk; }
        GameAction::Stop
    }
}
