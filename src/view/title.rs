use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, RenderArgs, Transformed, UpdateArgs};
use crate::app::{Direction, HeldKeys, Input};
use crate::circle_wipe::CircleWipe;
use crate::color::Color;
use crate::entity::Player;
use crate::scene::{Scene, CameraMode};
use crate::scene_config::SceneConfig;
use crate::view::Transition;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const ROOM_OFFSET_X: f64 = 20.;
const ROOM_OFFSET_Y: f64 = 24.;
const LOGO_LEFT_SRC: [f64; 4] = [0., 80., 80., 80.];
const LOGO_LEFT_DEST: [f64; 4] = [4., 9., 80., 80.];
const LOGO_RIGHT_SRC: [f64; 4] = [80., 112., 112., 48.];
const LOGO_RIGHT_DEST: [f64; 4] = [84., 41., 112., 48.];
const INPUT_CHECK_SRC: [f64; 4] = [48., 48., 80., 16.];
const INPUT_CHECK_DEST: [f64; 4] = [96. - ROOM_OFFSET_X, 112. - ROOM_OFFSET_Y, 80., 16.];
const PLAY_NOW_SRC: [f64; 4] = [48., 0., 80., 16.];
const PLAY_NOW_DEST: [f64; 4] = [96. - ROOM_OFFSET_X, 112. - ROOM_OFFSET_Y, 80., 16.];
const LEVELS_SRC: [f64; 4] = [48., 16., 48., 16.];
const LEVELS_DEST: [f64; 4] = [96. - ROOM_OFFSET_X, 128. - ROOM_OFFSET_Y, 48., 16.];
const CREDITS_SRC: [f64; 4] = [48., 32., 80., 16.];
const CREDITS_DEST: [f64; 4] = [96. - ROOM_OFFSET_X, 144. - ROOM_OFFSET_Y, 80., 16.];
const AUTHOR_SRC: [f64; 4] = [48., 64., 80., 16.];
const AUTHOR_DEST: [f64; 4] = [110., 184., 80., 16.];

enum State {
    InputCheck,
    Menu,
}

pub struct TitleView {
    texture: GlTexture,
    scene: Scene,
    cursor: Player,
    state: State,
    fade: Option<CircleWipe>,
    staged_transition: Option<Transition>,
}

impl TitleView {
    pub fn new() -> Self {
        let game = SceneConfig::new_title();
        let camera_mode = CameraMode::offset(ROOM_OFFSET_X as i64, ROOM_OFFSET_Y as i64);

        Self {
            texture: crate::app::load_texture(),
            scene: Scene::new(game, camera_mode),
            cursor: Player::new_cursor(0, 0, 16., 16.),
            state: State::InputCheck,
            fade: None,
            staged_transition: None,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.scene.prerender_lights(args, gl);

        gl.draw(args.viewport(), |_, gl| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], gl);
            self.render_scene(gl);
        });
    }

    pub fn render_scene(&self, gl: &mut GlGraphics) {
        let context = Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let room_context = context.trans(-ROOM_OFFSET_X, -ROOM_OFFSET_Y);
        let cursor_context = room_context.trans(80., 112.);

        let draw_state = if let Some(fade) = &self.fade {
            fade.render(&cursor_context, gl);
            DrawState::new_inside()
        }
        else { DrawState::default() };

        self.scene.render_stuff(&draw_state, gl);

        let mut draw_sprite = |src: [f64; 4], dest: [f64; 4]| {
            Image::new().src_rect(src).rect(dest).draw(
                &self.texture,
                &draw_state,
                context.transform,
                gl,
            );
        };
        draw_sprite(LOGO_LEFT_SRC, LOGO_LEFT_DEST);
        draw_sprite(LOGO_RIGHT_SRC, LOGO_RIGHT_DEST);
        match self.state {
            State::InputCheck => {
                draw_sprite(INPUT_CHECK_SRC, INPUT_CHECK_DEST);
            },
            State::Menu => {
                draw_sprite(PLAY_NOW_SRC, PLAY_NOW_DEST);
                draw_sprite(LEVELS_SRC, LEVELS_DEST);
                draw_sprite(CREDITS_SRC, CREDITS_DEST);
                draw_sprite(AUTHOR_SRC, AUTHOR_DEST);
                self.cursor.sprite().draw(
                    &self.texture,
                    &draw_state,
                    cursor_context.transform,
                    gl,
                );
            },
        }
        self.scene.render_lights(&draw_state, gl);
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.scene.update(args);

        if let Some(fade) = &mut self.fade {
            fade.update(args);
            if fade.done() {
                self.fade = None;
                return self.staged_transition.take();
            }
            return None;
        }
        match &self.state {
            State::InputCheck => { self.update_input_check(args, held_keys); },
            State::Menu => { self.update_menu(args, held_keys); },
        }
        None
    }

    fn update_input_check(&mut self, _args: &UpdateArgs, held_keys: &mut HeldKeys) {
        for input in held_keys.inputs() {
            if matches!(input, Input::Accept) {
                self.state = State::Menu;
            }
        }
    }

    fn update_menu(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) {
        self.cursor.update(args);
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction @ Direction::North)
                if self.cursor.y != 0 && self.cursor.can_walk() => {
                    self.cursor.walk(direction);
                },
                Input::Navigate(direction @ Direction::South)
                if self.cursor.y != 2 && self.cursor.can_walk() => {
                    self.cursor.walk(direction);
                },
                Input::Accept => match self.cursor.y {
                    0 => { self.fade_out(Transition::Game(0)); },
                    1 => { self.fade_out(Transition::Menu(0)); },
                    _ => (),
                },
                _ => ()
            }
        }
        let color = match self.cursor.y {
            0 => { Color::BLUE },
            1 => { Color::GREEN },
            2 => { Color::RED },
            _ => unreachable!(),
        };
        self.scene.set_light_color(color);
    }

    fn fade_out(&mut self, transition: Transition) {
        let (x, y) = self.cursor.center();
        self.fade = Some(CircleWipe::new_in(x as f64, y as f64));
        self.staged_transition = Some(transition);
    }
}
