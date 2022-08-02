use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, RenderArgs, Transformed, UpdateArgs};
use crate::app::{Direction, HeldKeys, Input};
use crate::circle_wipe::CircleWipe;
use crate::decal::Decal;
use crate::scene::{Scene, Camera};
use crate::scene_config::SceneConfig;
use crate::view::Transition;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const ROOM_OFFSET_X: f64 = 20.;
const ROOM_OFFSET_Y: f64 = 24.;

const LOGO_LEFT: Decal = Decal {
    src_left: 0., src_top: 80.,
    dest_left: 4., dest_top: 9.,
    width: 80., height: 80.,
};
const LOGO_RIGHT: Decal = Decal {
    src_left: 80., src_top: 112.,
    dest_left: 84., dest_top: 41.,
    width: 112., height: 48.,
};
const INPUT_CHECK: Decal = Decal {
    src_left: 48., src_top: 48.,
    dest_left: 96. - ROOM_OFFSET_X, dest_top: 112. - ROOM_OFFSET_Y,
    width: 80., height: 16.,
};
const PLAY_NOW: Decal = Decal {
    src_left: 48., src_top: 0.,
    dest_left: 96. - ROOM_OFFSET_X, dest_top: 112. - ROOM_OFFSET_Y,
    width: 80., height: 16.,
};
const LEVELS: Decal = Decal {
    src_left: 48., src_top: 16.,
    dest_left: 96. - ROOM_OFFSET_X, dest_top: 128. - ROOM_OFFSET_Y,
    width: 48., height: 16.,
};
const CREDITS: Decal = Decal {
    src_left: 48., src_top: 32.,
    dest_left: 96. - ROOM_OFFSET_X, dest_top: 144. - ROOM_OFFSET_Y,
    width: 80., height: 16.,
};
const AUTHOR: Decal = Decal {
    src_left: 48., src_top: 64.,
    dest_left: 110., dest_top: 184.,
    width: 80., height: 16.,
};

enum State {
    InputCheck,
    Menu,
}

pub struct TitleView {
    texture: GlTexture,
    scene: Scene,
    state: State,
    fade: Option<CircleWipe>,
    staged_transition: Option<Transition>,
}

impl TitleView {
    pub fn new() -> Self {
        let mut game = SceneConfig::new_title();
        game.camera = Camera::offset(ROOM_OFFSET_X as i64, ROOM_OFFSET_Y as i64);

        Self {
            texture: crate::app::load_texture(),
            scene: Scene::new(game),
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

        let draw_state = if let Some(fade) = &self.fade {
            fade.render(&room_context, gl);
            DrawState::new_inside()
        }
        else { DrawState::default() };

        self.scene.render_stuff(&draw_state, gl);

        let mut draw_decal = |decal: Decal| {
            decal.sprite().draw(
                &self.texture,
                &draw_state,
                context.transform,
                gl,
            );
        };
        draw_decal(LOGO_LEFT);
        draw_decal(LOGO_RIGHT);
        match self.state {
            State::InputCheck => {
                draw_decal(INPUT_CHECK);
            },
            State::Menu => {
                draw_decal(PLAY_NOW);
                draw_decal(LEVELS);
                draw_decal(CREDITS);
                draw_decal(AUTHOR);
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
            State::Menu => { self.update_menu(held_keys); },
        }
        None
    }

    fn update_input_check(&mut self, _args: &UpdateArgs, held_keys: &mut HeldKeys) {
        for input in held_keys.inputs() {
            if matches!(input, Input::Accept) {
                self.state = State::Menu;
                // Walk west, but immediately face east
                self.scene.navigate(Direction::West);
                self.scene.navigate(Direction::East);
            }
        }
    }

    fn update_menu(&mut self, held_keys: &mut HeldKeys) {
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction @ (Direction::North | Direction::South)) => {
                    self.scene.navigate(direction);
                }
                Input::Accept => match self.scene.player().y {
                    7 => { self.fade_out(Transition::Game(0)); },
                    8 => { self.fade_out(Transition::Menu(0)); },
                    _ => (),
                }
                _ => ()
            }
        }
    }

    fn fade_out(&mut self, transition: Transition) {
        let (x, y) = self.scene.player().center();
        self.fade = Some(CircleWipe::new_in(x as f64, y as f64));
        self.staged_transition = Some(transition);
    }
}
