use crate::view::{Transition, View};
use piston_window::{Button, Key};
use piston_window::{clear, RenderArgs, UpdateArgs};
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

const AMBIENT_LUM: f32 = 0.6;

pub fn lerp<T>(left: [T; 4], right: [T; 4], p: T) -> [T; 4]
where T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy {  // lmao
    [
        (right[0] - left[0]) * p + left[0],
        (right[1] - left[1]) * p + left[1],
        (right[2] - left[2]) * p + left[2],
        (right[3] - left[3]) * p + left[3],
    ]
}

pub fn int_lerp(left: [f64; 4], right: [f64; 4], p: f64) -> [f64; 4] {
    [
        ((right[0] - left[0]) * p + left[0]) as i64 as f64,
        ((right[1] - left[1]) * p + left[1]) as i64 as f64,
        ((right[2] - left[2]) * p + left[2]) as i64 as f64,
        ((right[3] - left[3]) * p + left[3]) as i64 as f64,
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
}

impl App {
    pub fn new() -> Self {
        App {
            view: View::title(),
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
        match self.view.update(args, &mut self.held_keys) {
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
                Button::Keyboard(Key::Space | Key::Z) => Input::Accept,
                Button::Keyboard(Key::Backspace) => Input::Reject,
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
