use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Button, Key};
use piston_window::{Context, DrawState, UpdateArgs, Image, Transformed};
use image;
use crate::app::HeldKeys;
use crate::player::Player;
use crate::room::Room;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const DISPLAY_WIDTH_HALF: i64 = DISPLAY_WIDTH as i64 / 2;
const DISPLAY_HEIGHT_HALF: i64 = DISPLAY_HEIGHT as i64 / 2;

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
    room: Room,
}

impl GameView {
    pub fn new() -> Self {
        GameView {
            texture: load_texture(),
            player: Player::new(),
            room: Room::new(),
        }
    }

    fn absolute_context(&self) -> Context {
        Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    fn camera_context(&self) -> Context {
        let (x, y) = self.camera();
        let x = x as f64;
        let y = y as f64;
        self.absolute_context()
            .trans(-x + DISPLAY_WIDTH / 2., -y + DISPLAY_HEIGHT / 2.)
    }

    // TODO: if the level's too small probably center it instead
    fn camera(&self) -> (i64, i64) {
        let (x, y) = self.player.center();
        let mut xs = vec![DISPLAY_WIDTH_HALF, x, self.room.pixel_width() - DISPLAY_WIDTH_HALF];
        let mut ys = vec![DISPLAY_HEIGHT_HALF, y, self.room.pixel_height() - DISPLAY_HEIGHT_HALF];
        xs.sort();
        ys.sort();
        (xs[1], ys[1])
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let context = self.camera_context();

        self.room.render(
            &self.texture,
            &DrawState::default(),
            &context,
            gl,
        );
        Image::new()
            .src_rect(BLOCK)
            .rect([16., 16., 16., 24.])
            .draw(
                &self.texture,
                &DrawState::default(),
                context.transform,
                gl,
            );

        Image::new()
            .src_rect(BLOCK)
            .rect([48., 16., 16., 24.])
            .draw(
                &self.texture,
                &DrawState::default(),
                context.transform,
                gl,
            );

        self.player.sprite()
            .draw(
                &self.texture,
                &DrawState::default(),
                context.transform,
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
