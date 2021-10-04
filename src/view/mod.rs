use opengl_graphics::GlGraphics;
use piston_window::UpdateArgs;
use crate::app::HeldKeys;

pub mod game;
pub mod menus;

pub use game::{GameView, GameAction};
pub use menus::MenuView;

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

pub enum Transition {
    Game(usize),
    Menu(usize),
}

pub enum View {
    Menu(MenuView),
    Game(GameView),
}

impl View {
    pub fn menu(level_id: usize) -> Self {
        Self::Menu(MenuView::new(level_id))
    }

    pub fn game(level_id: usize) -> Self {
        Self::Game(GameView::new(level_id))
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        match self {
            View::Menu(v) => v.render(gl),
            View::Game(v) => v.render(gl),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &HeldKeys) -> Option<Transition> {
        match self {
            View::Menu(v) => v.update(args, held_keys),
            View::Game(v) => v.update(args, held_keys),
        }
    }
}
