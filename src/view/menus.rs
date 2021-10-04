use crate::app::{HeldKeys, Input};
use crate::entity::Player;
use crate::view::Transition;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, Rectangle, Transformed, UpdateArgs};
use piston_window::rectangle::rectangle_by_corners;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const LEVEL_PADDING: f64 = 8.;
const LEVELS_HORIZONTAL: usize = 5;
const LEVELS_VERTICAL: usize = 2;
const LEVEL_WIDTH: f64 = 30.;
const LEVEL_HEIGHT: f64 = 25.;
const LEVEL_OFFSET_X: f64 = 9.;
const LEVEL_OFFSET_Y: f64 = 9.;
pub const LEVEL_SPACING_X: f64 = LEVEL_WIDTH + LEVEL_PADDING;
pub const LEVEL_SPACING_Y: f64 = LEVEL_HEIGHT + LEVEL_PADDING;

pub struct MenuView {
    texture: GlTexture,
    completed_levels: Vec<usize>,
    cursor: Player,
}

impl MenuView {
    pub fn new(level: usize, completed_levels: Vec<usize>) -> Self {
        let x = level % LEVELS_HORIZONTAL;
        let y = level / LEVELS_HORIZONTAL;
        Self {
            texture: crate::app::load_texture(),
            cursor: Player::new_cursor(x as i32, y as i32, LEVEL_SPACING_X, LEVEL_SPACING_Y),
            completed_levels,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let context = Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let color = Rectangle::new([0.2, 0.2, 0.2, 1.]);
        for y in 0..LEVELS_VERTICAL {
            for x in 0..LEVELS_HORIZONTAL {
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

        for idx in &self.completed_levels {
            let x = idx % LEVELS_HORIZONTAL;
            let y = idx / LEVELS_HORIZONTAL;
            let left = LEVEL_OFFSET_X + x as f64 * LEVEL_SPACING_X + 20.;
            let top = LEVEL_OFFSET_Y + y as f64 * LEVEL_SPACING_Y - 5.;
            Image::new()
                .src_rect([96., 16., 16., 16.])
                .rect([left, top, 16., 16.])
                .draw(
                    &self.texture,
                    &DrawState::default(),
                    context.transform,
                    gl,
                );
        }

        self.cursor.sprite().draw(
            &self.texture,
            &DrawState::default(),
            context.trans(15., 15.).transform,
            gl,
        );
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.cursor.update(args);
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction) => {
                    self.cursor.face(&direction);
                    let (nx, ny) = direction.from(self.cursor.x, self.cursor.y);
                    if self.cursor.can_walk() && nx >= 0 && nx < 5 && ny >= 0 && ny < 5  {
                        self.cursor.walk(&direction);
                    }
                }
                Input::Accept => {
                    let level_id = self.cursor.y as usize * LEVELS_HORIZONTAL + self.cursor.x as usize;
                    return Some(Transition::Game(level_id));
                }
                _ => (),
            }
        }
        None
    }
}
