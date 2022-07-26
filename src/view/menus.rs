use crate::app::{HeldKeys, Input};
use crate::circle_wipe::CircleWipe;
use crate::entity::Player;
use crate::view::Transition;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, Rectangle, RenderArgs, Transformed, UpdateArgs};
use piston_window::rectangle::rectangle_by_corners;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const LEVEL_PADDING: f64 = 8.;
const LEVELS_HORIZONTAL: usize = 4;
const LEVELS_VERTICAL: usize = 2;
const LEVEL_WIDTH: f64 = 30.;
const LEVEL_HEIGHT: f64 = 25.;
const LEVEL_OFFSET_X: f64 = 29.;
const LEVEL_OFFSET_Y: f64 = 9.;
const LEVEL_SPACING_X: f64 = LEVEL_WIDTH + LEVEL_PADDING;
const LEVEL_SPACING_Y: f64 = LEVEL_HEIGHT + LEVEL_PADDING;
const INSTRUCTION_SRC: [f64; 4] = [192., 112., 64., 48.];
const INSTRUCTION_DEST: [f64; 4] = [68., 100., 64., 48.];

pub struct MenuView {
    texture: GlTexture,
    completed_levels: Vec<usize>,
    cursor: Player,
    fade: Option<CircleWipe>,
    staged_transition: Option<Transition>,
}

impl MenuView {
    pub fn new(level: usize, completed_levels: Vec<usize>) -> Self {
        let x = level % LEVELS_HORIZONTAL;
        let y = level / LEVELS_HORIZONTAL;
        let cursor = Player::new_cursor(x as i32, y as i32, LEVEL_SPACING_X, LEVEL_SPACING_Y);
        let (cx, cy) = cursor.center();
        Self {
            texture: crate::app::load_texture(),
            cursor,
            completed_levels,
            fade: Some(CircleWipe::new_out(cx as f64, cy as f64)),
            staged_transition: None,
        }
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |_, gl| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], gl);
            self.render_scene(gl);
        });
    }

    pub fn render_scene(&self, gl: &mut GlGraphics) {
        let context = Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let cursor_context = context.trans(35., 15.);
        let color = Rectangle::new([0.7, 0.7, 0.7, 1.]);

        let draw_state = if let Some(fade) = &self.fade {
            fade.render(&cursor_context, gl);
            DrawState::new_inside()
        }
        else { DrawState::default() };

        Rectangle::new([0.3, 0.3, 0.3, 1.0]).draw(
            [0., 0., DISPLAY_WIDTH, DISPLAY_HEIGHT],
            &draw_state,
            context.transform,
            gl,
        );
        for y in 0..LEVELS_VERTICAL {
            for x in 0..LEVELS_HORIZONTAL {
                let left = LEVEL_OFFSET_X + x as f64 * LEVEL_SPACING_X;
                let top = LEVEL_OFFSET_Y + y as f64 * LEVEL_SPACING_Y;
                let right = left + LEVEL_WIDTH;
                let bottom = top + LEVEL_HEIGHT;
                color.draw(
                    rectangle_by_corners(left, top, right, bottom),
                    &draw_state,
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
                    &draw_state,
                    context.transform,
                    gl,
                );
        }

        Image::new()
            .src_rect(INSTRUCTION_SRC)
            .rect(INSTRUCTION_DEST)
            .draw(
                &self.texture,
                &draw_state,
                context.transform,
                gl,
            );

        self.cursor.sprite().draw(
            &self.texture,
            &draw_state,
            cursor_context.transform,
            gl,
        );
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.cursor.update(args);
        if let Some(fade) = &mut self.fade {
            fade.update(args);
            if fade.done() {
                self.fade = None;
                return self.staged_transition.take();
            }
            return None;
        }
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction) => {
                    self.cursor.face(direction);
                    let (nx, ny) = direction.from(self.cursor.x, self.cursor.y);
                    if self.cursor.can_walk()
                            && nx >= 0 && nx < LEVELS_HORIZONTAL as i32
                            && ny >= 0 && ny < LEVELS_VERTICAL as i32  {
                        self.cursor.walk(direction);
                    }
                }
                Input::Accept => {
                    let level_id = self.cursor.y as usize * LEVELS_HORIZONTAL + self.cursor.x as usize;
                    self.fade_out(Transition::Game(level_id));
                }
                _ => (),
            }
        }
        None
    }

    fn fade_out(&mut self, transition: Transition) {
        let (x, y) = self.cursor.center();
        self.fade = Some(CircleWipe::new_in(x as f64, y as f64));
        self.staged_transition = Some(transition);
    }
}
