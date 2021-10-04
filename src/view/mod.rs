pub mod game;
pub mod menus;

#[derive(Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    pub fn from(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            North => (x, y - 1),
            West => (x - 1, y),
            South => (x, y + 1),
            East => (x + 1, y),
        }
    }
}

pub use game::{GameView, GameAction};
pub use menus::MenuView;
