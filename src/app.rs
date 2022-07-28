use std::collections::HashSet;
use crate::view::{Transition, View};
use piston_window::{Button, Key};
use piston_window::{RenderArgs, UpdateArgs};
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

pub fn lerp<T>(left: T, right: T, p: T) -> T
where T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy {
    (right - left) * p + left
}

pub fn lerpn<T, const N: usize>(left: [T; N], right: [T; N], p: T) -> [T; N]
where T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy {
    let mut i = 0;
    left.map(|a| {
        let b = right[i];
        i += 1;
        lerp(a, b, p)
    })
}

pub fn int_lerp4(left: [f64; 4], right: [f64; 4], p: f64) -> [f64; 4] {
    [
        lerp(left[0], right[0], p) as i64 as f64,
        lerp(left[1], right[1], p) as i64 as f64,
        lerp(left[2], right[2], p) as i64 as f64,
        lerp(left[3], right[3], p) as i64 as f64,
    ]
}

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
    completed_levels: HashSet<usize>,  // haha wow this probably shouldn't go here
}

impl App {
    pub fn new() -> Self {
        App {
            view: View::title(),
            held_keys: HeldKeys::new(),
            completed_levels: HashSet::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.view.render(args, gl);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.view.update(args, &mut self.held_keys) {
            Some(Transition::Game(level_id)) => {
                let top = crate::scene_config::NUM_LEVELS - 1;
                if level_id > top { self.nav_to_menu(top); }
                else { self.view = View::game(level_id); }
            },
            Some(Transition::Menu(level_id)) => { self.nav_to_menu(level_id); }
            Some(Transition::Win(level_id)) => {
                self.completed_levels.insert(level_id);
            }
            None => (),
        }
    }

    fn nav_to_menu(&mut self, level_id: usize) {
        let completed_levels = self.completed_levels.iter().copied().collect();
        self.view = View::menu(level_id, completed_levels);
    }

    pub fn key_press(&mut self, button: &Button) {
        self.held_keys.hold(button);
    }

    pub fn key_release(&mut self, button: &Button) {
        self.held_keys.release(button);
    }
}

#[derive(PartialEq, Clone, Copy)]
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

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
        }
    }
}

#[derive(PartialEq)]
pub enum Input {
    Navigate(Direction),
    Accept,
    Reject,
    Help,
}

pub struct HeldKeys {
    ordered_keys: Vec<Button>,
}

impl HeldKeys {
    fn new() -> Self {
        HeldKeys {
            ordered_keys: Vec::new(),
        }
    }

    fn hold(&mut self, button: &Button) {
        if !self.ordered_keys.contains(button) {
            self.ordered_keys.push(*button);
        }
    }

    fn release(&mut self, button: &Button) {
        if let Some(index) = self.ordered_keys.iter().position(|x| x == button) {
            self.ordered_keys.remove(index);
        }
    }

    pub fn inputs(&mut self) -> Vec<Input> {
        let mut inputs = Vec::new();
        let mut i = self.ordered_keys.len();
        while i != 0 {
            i -= 1;
            let key = &self.ordered_keys[i];
            let input = match key {
                Button::Keyboard(Key::W | Key::Up) => Input::Navigate(Direction::North),
                Button::Keyboard(Key::A | Key::Left) => Input::Navigate(Direction::West),
                Button::Keyboard(Key::S | Key::Down) => Input::Navigate(Direction::South),
                Button::Keyboard(Key::D | Key::Right) => Input::Navigate(Direction::East),
                Button::Keyboard(Key::Space | Key::Return | Key::Z) => Input::Accept,
                Button::Keyboard(Key::Backspace) => Input::Reject,
                Button::Keyboard(Key::H) => Input::Help,
                _ => continue,
            };
            // Evict inputs which should not turbo
            if matches!(&input, Input::Accept | Input::Reject) {
                self.ordered_keys.remove(i);
            }
            if !inputs.contains(&input) {
                inputs.push(input);
            }
        }
        inputs
    }
}
