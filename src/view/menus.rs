use crate::app::HeldKeys;
use crate::entity::Player;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Button, Key};
use piston_window::{Context, DrawState, Rectangle, UpdateArgs};
use piston_window::rectangle::rectangle_by_corners;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const LEVEL_PADDING: f64 = 8.;
const LEVEL_WIDTH: f64 = 30.;
const LEVEL_HEIGHT: f64 = 25.;
const LEVEL_OFFSET_X: f64 = 9.;
const LEVEL_OFFSET_Y: f64 = 9.;
pub const LEVEL_SPACING_X: f64 = LEVEL_WIDTH + LEVEL_PADDING;
pub const LEVEL_SPACING_Y: f64 = LEVEL_HEIGHT + LEVEL_PADDING;

pub struct MenuView {
    texture: GlTexture,
    cursor: Player,
}

impl MenuView {
    pub fn new() -> Self {
        Self {
            texture: crate::app::load_texture(),
            cursor: Player::new(0, 0),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let context = Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let color = Rectangle::new([0.2, 0.2, 0.2, 1.]);
        for y in 0..5 {
            for x in 0..5 {
                let left = LEVEL_OFFSET_X + x as f64 * LEVEL_SPACING_X;
                let top = LEVEL_OFFSET_Y + y as f64 * LEVEL_SPACING_Y;
                let right = left + LEVEL_WIDTH;
                let bottom = top + LEVEL_HEIGHT;
                color.draw(
                    rectangle_by_corners(left, top, right, bottom),
                    &DrawState::default(),
                    context.transform,
                    gl,
                );
            }
        }

        self.cursor.menu_sprite().draw(
            &self.texture,
            &DrawState::default(),
            context.transform,
            gl,
        );
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &HeldKeys) {
        self.cursor.update(args);
        use crate::view::Direction;
        for key in held_keys.iter() {
            let maybe_direction = match key {
                Button::Keyboard(Key::Space) => {
                    return;
                },
                Button::Keyboard(Key::W) => Some(Direction::North),
                Button::Keyboard(Key::A) => Some(Direction::West),
                Button::Keyboard(Key::S) => Some(Direction::South),
                Button::Keyboard(Key::D) => Some(Direction::East),
                _ => None,
            };
            if let Some(direction) = maybe_direction {
                self.cursor.face(&direction);
                let (nx, ny) = direction.from(self.cursor.x, self.cursor.y);
                if self.cursor.can_walk() {
                    self.cursor.walk(&direction);
                }
            }
        }
    }
}
