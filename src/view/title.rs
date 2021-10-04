use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Transformed, UpdateArgs};
use crate::app::{HeldKeys, Input};
use crate::view::Transition;
use crate::entity::Player;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;

pub struct TitleView {
    texture: GlTexture,
    cursor: Player,
}

impl TitleView {
    pub fn new() -> Self {
        Self {
            texture: crate::app::load_texture(),
            cursor: Player::new_cursor(0, 0, 0., 0.)
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let context = Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        self.cursor.sprite().draw(
            &self.texture,
            &DrawState::default(),
            context.trans(15., 15.).transform,
            gl,
        );
    }
    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        for input in held_keys.inputs() {
            match input {
                Input::Accept => { return Some(Transition::Menu(0)); },
                _ => ()
            }
        }
        None
    }
}
