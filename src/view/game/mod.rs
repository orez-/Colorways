mod thought;

use crate::app::{int_lerp4, Direction, HeldKeys, Input};
use crate::decal::Decal;
use crate::circle_wipe::CircleWipe;
use crate::entity::Player;
use crate::scene::{Scene, SceneTag, CameraMode, HistoryEvent, HistoryEventType};
use crate::scene_config::SceneConfig;
use crate::view::game::thought::Thought;
use crate::view::Transition;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, RenderArgs, Transformed, UpdateArgs};

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;

const LEVEL_COMPLETE_SRC: [f64; 4] = [128., 0., 128., 112.];
const LEVEL_COMPLETE_START_DEST: [f64; 4] = [36., -112., 128., 112.];
const LEVEL_COMPLETE_END_DEST: [f64; 4] = [36., 40., 128., 112.];

const MOVE_LEFT: f64 = 80.;
const MOVE_TOP: f64 = 136.;
const MOVE_OPTIONS: Decal = Decal {
    src_left: 0., src_top: 160.,
    dest_left: MOVE_LEFT, dest_top: MOVE_TOP,
    width: 112., height: 32.,
};
const MOVE_TEXT: Decal = Decal {
    src_left: 0., src_top: 192.,
    dest_left: MOVE_LEFT + 40., dest_top: MOVE_TOP + 32.,
    width: 32., height: 16.,
};
const UNDO: Decal = Decal {
    src_left: 112., src_top: 160.,
    dest_left: 216., dest_top: 40.,
    width: 32., height: 32.,
};

pub enum State {
    Play,
    Win(f64),
}

pub struct GameView {
    texture: GlTexture,
    scene: Scene,
    level_id: usize,
    cursor: Option<Player>,
    state: State,
    thought: Thought,
    history: Vec<HistoryEvent>,
    fade: Option<CircleWipe>,
    staged_transition: Option<Transition>,
}

impl GameView {
    pub fn new(level_id: usize) -> Self {
        let scene_config = SceneConfig::new(level_id);
        let camera_mode = CameraMode::Player;
        let scene = Scene::new(scene_config, camera_mode);
        let (cx, cy) = scene.player().center();
        GameView {
            texture: crate::app::load_texture(),
            scene,
            level_id,
            cursor: None,
            state: State::Play,
            thought: Thought::new(),
            history: Vec::new(),
            fade: Some(CircleWipe::new_out(cx as f64, cy as f64)),
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

    fn render_game(&self, draw_state: &DrawState, gl: &mut GlGraphics) {
        self.scene.render_stuff(draw_state, gl);

        let context = self.scene.camera_context();
        let mut draw_decal = |decal: Decal| {
            decal.sprite().draw(
                &self.texture,
                draw_state,
                context.transform,
                gl,
            );
        };
        match self.scene.tag {
            Some(SceneTag::TeachMove) => {
                draw_decal(MOVE_OPTIONS);
                draw_decal(MOVE_TEXT);
            }
            Some(SceneTag::TeachUndo) => {
                draw_decal(UNDO);
            }
            None => (),
        }

        self.scene.render_lights(draw_state, gl);

        // Thoughts
        let (px, py) = self.scene.player().pixel_coord();
        self.thought.render(
            &self.texture,
            draw_state,
            &context.trans(px, py),
            gl,
        );
    }

    fn absolute_context(&self) -> Context {
        Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    pub fn render_scene(&mut self, gl: &mut GlGraphics) {
        let abs_context = self.absolute_context();
        let cursor_context = abs_context.trans(46., 107.);

        let draw_state = if let Some(fade) = &self.fade {
            let fade_context = if self.cursor.is_some() { cursor_context }
                else { self.scene.camera_context() };
            fade.render(&fade_context, gl);
            DrawState::new_inside()
        }
        else { DrawState::default() };

        self.render_game(&draw_state, gl);

        if let State::Win(progress) = self.state {
            let dest = int_lerp4(LEVEL_COMPLETE_START_DEST, LEVEL_COMPLETE_END_DEST, progress);
            Image::new().src_rect(LEVEL_COMPLETE_SRC).rect(dest).draw(
                &self.texture,
                &draw_state,
                abs_context.transform,
                gl,
            );
            if let Some(cursor) = &self.cursor {
                cursor.sprite().draw(
                    &self.texture,
                    &draw_state,
                    cursor_context.transform,
                    gl,
                );
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.scene.update(args);
        self.thought.update(args);

        if let Some(fade) = &mut self.fade {
            fade.update(args);
            if fade.done() {
                self.fade = None;
                return self.staged_transition.take();
            }
            return None;
        }

        match &mut self.state {
            State::Play => self.update_play(held_keys),
            State::Win(progress) => {
                if *progress < 1. {
                    *progress += args.dt * 5.;
                    if *progress >= 1. {
                        *progress = 1.;
                        self.cursor = Some(Player::new(0, 0));
                    }
                }
                self.update_win(args, held_keys)
            }
        }
    }

    fn update_play(&mut self, held_keys: &mut HeldKeys) -> Option<Transition> {
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction) => {
                    self.thought.dismiss();
                    match self.scene.navigate(direction) {
                        Some(HistoryEvent { event_type: HistoryEventType::Win, .. }) => {
                            self.state = State::Win(0.);
                            return Some(Transition::Win(self.level_id));
                        }
                        Some(evt) => { self.history.push(evt); }
                        None => (),
                    }
                }
                Input::Accept => { self.fade_out(Transition::Menu(self.level_id)); }
                Input::Reject => { self.undo(); }
                Input::Help => { self.thought.think(); }
            }
        }
        None
    }

    fn update_win(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        let mut transition = None;
        if let Some(cursor) = &mut self.cursor {
            cursor.update(args);
            for input in held_keys.inputs() {
                if !cursor.can_walk() {
                    break;
                }
                match input {
                    Input::Navigate(direction @ Direction::North) if cursor.y == 1 => {
                        cursor.walk(direction);
                    }
                    Input::Navigate(direction @ Direction::South) if cursor.y == 0 => {
                        cursor.walk(direction);
                    }
                    Input::Accept => match cursor.y {
                        0 => { transition = Some(Transition::Game(self.level_id + 1)); }
                        1 => { transition = Some(Transition::Menu(self.level_id)); }
                        _ => (),
                    },
                    _ => (),
                }
            }
        };
        if let Some(t) = transition { self.fade_out(t); }
        None
    }

    fn fade_out(&mut self, transition: Transition) {
        let (x, y) = self.cursor.as_ref().unwrap_or(&self.scene.player()).center();
        self.fade = Some(CircleWipe::new_in(x as f64, y as f64));
        self.staged_transition = Some(transition);
    }

    fn undo(&mut self) {
        let event = match self.history.pop() {
            Some(value) => value,
            None => return,
        };
        self.scene.undo(event);
    }
}
