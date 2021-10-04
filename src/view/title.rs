use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, Transformed, UpdateArgs};
use crate::app::{Direction, HeldKeys, Input};
use crate::color::Color;
use crate::entity::{Entity, Player};
use crate::room::Room;
use crate::view::Transition;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const ROOM_OFFSET_X: f64 = -20.;
const ROOM_OFFSET_Y: f64 = -24.;
const LOGO_LEFT_SRC: [f64; 4] = [0., 80., 80., 80.];
const LOGO_LEFT_DEST: [f64; 4] = [4., 9., 80., 80.];
const LOGO_RIGHT_SRC: [f64; 4] = [80., 112., 112., 48.];
const LOGO_RIGHT_DEST: [f64; 4] = [84., 41., 112., 48.];
const INPUT_CHECK_SRC: [f64; 4] = [48., 48., 80., 16.];
const INPUT_CHECK_DEST: [f64; 4] = [96. + ROOM_OFFSET_X, 112. + ROOM_OFFSET_Y, 80., 16.];
const PLAY_NOW_SRC: [f64; 4] = [48., 0., 80., 16.];
const PLAY_NOW_DEST: [f64; 4] = [96. + ROOM_OFFSET_X, 112. + ROOM_OFFSET_Y, 80., 16.];
const LEVELS_SRC: [f64; 4] = [48., 16., 80., 16.];
const LEVELS_DEST: [f64; 4] = [96. + ROOM_OFFSET_X, 128. + ROOM_OFFSET_Y, 80., 16.];
const CREDITS_SRC: [f64; 4] = [48., 32., 80., 16.];
const CREDITS_DEST: [f64; 4] = [96. + ROOM_OFFSET_X, 144. + ROOM_OFFSET_Y, 80., 16.];
const AUTHOR_SRC: [f64; 4] = [48., 64., 80., 16.];
const AUTHOR_DEST: [f64; 4] = [110., 184., 80., 16.];

enum State {
    InputCheck,
    Menu,
}

pub struct TitleView {
    texture: GlTexture,
    cursor: Player,
    room: Room,
    entities: Vec<Entity>,
    light_color: Color,
    state: State,
}

impl TitleView {
    pub fn new() -> Self {
        let (room, _, entities, light_color) = Room::new_title();
        let mut title = Self {
            texture: crate::app::load_texture(),
            cursor: Player::new_cursor(0, 0, 16., 16.),
            room, entities, light_color: Color::Gray,
            state: State::InputCheck,
        };
        title.set_light_color(light_color);
        title
    }

    fn render_lights(&self, gl: &mut GlGraphics, context: &Context) {
        let lights: Vec<_> = self.entities.iter().filter_map(|e| {
            if let Entity::Lightbulb(bulb) = e { Some(bulb) }
            else { None }
        }).collect();

        for light in &lights {
            light.draw_light(context, gl);
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let context = Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let room_context = context.trans(ROOM_OFFSET_X, ROOM_OFFSET_Y);
        self.room.render(
            &self.texture,
            &DrawState::default(),
            &room_context,
            gl,
        );
        for entity in &self.entities {
            entity.sprite().draw(
                &self.texture,
                &DrawState::default(),
                room_context.transform,
                gl,
            );
        }
        let mut draw_sprite = |src: [f64; 4], dest: [f64; 4]| {
            Image::new().src_rect(src).rect(dest).draw(
                &self.texture,
                &DrawState::default(),
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
                    &DrawState::default(),
                    room_context.trans(80., 112.).transform,
                    gl,
                );
            },
        }
        self.render_lights(gl, &room_context);
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        for entity in self.entities.iter_mut() {
            entity.update(args);
        }
        match self.state {
            State::InputCheck => { self.update_input_check(args, held_keys); None },
            State::Menu => { self.update_menu(args, held_keys) },
        }
    }

    fn update_input_check(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) {
        for input in held_keys.inputs() {
            if matches!(input, Input::Accept) {
                self.state = State::Menu;
            }
        }
    }

    fn update_menu(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.cursor.update(args);
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction @ Direction::North)
                if self.cursor.y != 0 && self.cursor.can_walk() => {
                    self.cursor.walk(&direction);
                },
                Input::Navigate(direction @ Direction::South)
                if self.cursor.y != 2 && self.cursor.can_walk() => {
                    self.cursor.walk(&direction);
                },
                Input::Accept => match self.cursor.y {
                    0 => { return Some(Transition::Game(0)); },
                    1 => { return Some(Transition::Menu(0)); },
                    _ => (),
                },
                _ => ()
            }
        }
        let color = match self.cursor.y {
            0 => { Color::Blue },
            1 => { Color::Green },
            2 => { Color::Red },
            _ => unreachable!(),
        };
        self.set_light_color(color);
        None
    }

    pub fn set_light_color(&mut self, color: Color) {
        if self.light_color == color { return; }
        for entity in self.entities.iter_mut() {
            if let Entity::Lightbulb(bulb) = entity {
                if bulb.color == self.light_color { bulb.turn_off(); }
                else if bulb.color == color { bulb.turn_on(); }
            }
        }
        self.light_color = color;
    }
}
