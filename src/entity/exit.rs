use piston_window::Image;
use crate::app::Direction;
use crate::entity::IEntity;
use crate::scene::{Scene, GameAction};

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
}

impl IEntity for Exit {
    fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new()
            .src_rect(EXIT)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    fn on_approach(&self, _entity_id: usize, _direction: Direction, _game: &Scene) -> GameAction { GameAction::Win }
}
