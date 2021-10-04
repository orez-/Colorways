use std::collections::HashSet;
use crate::view::{Transition, View};
use piston_window::{Button, Key};
use piston_window::{clear, RenderArgs, UpdateArgs};
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

const AMBIENT_LUM: f32 = 0.6;

const TEXTURE: &[u8] = include_bytes!("../bin/spritesheet.png");

pub fn load_texture() -> GlTexture {
    let mut texture_settings = opengl_graphics::TextureSettings::new();
    texture_settings.set_mag(Filter::Nearest);

    let img = image::load_from_memory(TEXTURE).unwrap();
    let img = match img {
        image::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba8(),
    };
    GlTexture::from_image(&img, &texture_settings)
}

pub struct App {
    view: View,
    held_keys: HeldKeys,
}

impl App {
    pub fn new() -> Self {
        App {
            view: View::menu(0),
            held_keys: HeldKeys::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let v = args.viewport();
        gl.draw(v, |_, gl| {
            clear([AMBIENT_LUM, AMBIENT_LUM, AMBIENT_LUM, 1.0], gl);
            self.view.render(gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.view.update(args, &self.held_keys) {
            Some(Transition::Game(level_id)) => { self.view = View::game(level_id); },
            Some(Transition::Menu(level_id)) => { self.view = View::menu(level_id); },
            None => (),
        }
    }

    pub fn key_press(&mut self, button: &Button) {
        self.held_keys.hold(button);
    }

    pub fn key_release(&mut self, button: &Button) {
        self.held_keys.release(button);
    }
}

#[derive(PartialEq, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn from(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::West => (x - 1, y),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
        }
    }
}

#[derive(PartialEq)]
pub enum Input {
    Navigate(Direction),
    Accept,
    Reject,
}

pub struct HeldKeys {
    key_set: HashSet<Button>,
    ordered_keys: Vec<Button>,
}

impl HeldKeys {
    fn new() -> Self {
        HeldKeys {
            key_set: HashSet::new(),
            ordered_keys: Vec::new(),
        }
    }

    fn hold(&mut self, button: &Button) {
        if self.key_set.insert(*button) {
            self.ordered_keys.insert(0, *button);
        }
    }

    fn release(&mut self, button: &Button) {
        if self.key_set.remove(button) {
            let index = self.ordered_keys.iter().position(|x| x == button).unwrap();
            self.ordered_keys.remove(index);
        }
    }

    pub fn inputs(&self) -> Vec<Input> {
        let mut inputs = Vec::new();
        for key in &self.ordered_keys {
            let input = match key {
                Button::Keyboard(Key::Space | Key::Z) => Input::Accept,
                Button::Keyboard(Key::W) => Input::Navigate(Direction::North),
                Button::Keyboard(Key::A) => Input::Navigate(Direction::West),
                Button::Keyboard(Key::S) => Input::Navigate(Direction::South),
                Button::Keyboard(Key::D) => Input::Navigate(Direction::East),
                Button::Keyboard(Key::Backspace) => Input::Reject,
                _ => continue,
            };
            if !inputs.contains(&input) {
                inputs.push(input);
            }
        }
        inputs
    }
}
