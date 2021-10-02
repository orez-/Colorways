use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Button, Key};
use piston_window::{Context, DrawState, UpdateArgs, Image};
use image;
use crate::app::HeldKeys;
use crate::player::Player;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;

const TEXTURE: &[u8] = include_bytes!("../bin/spritesheet.png");
const BLOCK: [f64; 4] = [64., 0., 16., 24.];

fn load_texture() -> GlTexture {
    let mut texture_settings = opengl_graphics::TextureSettings::new();
    texture_settings.set_mag(Filter::Nearest);

    let img = image::load_from_memory(TEXTURE).unwrap();
    let img = match img {
        image::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba8(),
    };
    GlTexture::from_image(&img, &texture_settings)
}

#[derive(Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

pub struct GameView {
    texture: GlTexture,
    player: Player,
}

impl GameView {
    pub fn new() -> Self {
        GameView {
            texture: load_texture(),
            player: Player::new(),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        Image::new()
            .src_rect(BLOCK)
            .rect([16., 16., 16., 24.])
            .draw(
                &self.texture,
                &DrawState::default(),
                Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT).transform,
                gl,
            );

        Image::new()
            .src_rect(BLOCK)
            .rect([48., 16., 16., 24.])
            .draw(
                &self.texture,
                &DrawState::default(),
                Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT).transform,
                gl,
            );

        self.player.sprite()
            .draw(
                &self.texture,
                &DrawState::default(),
                Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT).transform,
                gl,
            );
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &HeldKeys) {
        self.player.update(args);
        for key in held_keys.iter() {
            let maybe_direction = match key {
                Button::Keyboard(Key::W) => Some(North),
                Button::Keyboard(Key::A) => Some(West),
                Button::Keyboard(Key::S) => Some(South),
                Button::Keyboard(Key::D) => Some(East),
                _ => None,
            };
            if let Some(direction) = maybe_direction {
                self.player.face(&direction);
                self.player.walk(&direction);
            }
        }
    }
}
