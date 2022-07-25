use opengl_graphics::GlGraphics;
use piston_window::UpdateArgs;
use crate::app::HeldKeys;

pub mod game;
pub mod menus;
pub mod title;

pub use game::{GameView, GameAction};
pub use menus::MenuView;
pub use title::TitleView;

pub enum Transition {
    Game(usize),
    Menu(usize),
    Win(usize),  // this isn't a transition at all! Who's running this circus??
}

pub enum View {
    Game(GameView),
    Menu(MenuView),
    Title(TitleView),
}

impl View {
    pub fn menu(level_id: usize, completed_levels: Vec<usize>) -> Self {
        Self::Menu(MenuView::new(level_id, completed_levels))
    }

    pub fn game(level_id: usize) -> Self {
        Self::Game(GameView::new(level_id))
    }

    pub fn title() -> Self {
        Self::Title(TitleView::new())
    }

    pub fn render(&mut self, gl: &mut GlGraphics) {
        match self {
            View::Menu(v) => v.render(gl),
            View::Game(v) => v.render(gl),
            View::Title(v) => v.render(gl),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        match self {
            View::Menu(v) => v.update(args, held_keys),
            View::Game(v) => v.update(args, held_keys),
            View::Title(v) => v.update(args, held_keys),
        }
    }
}
