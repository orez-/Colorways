use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Button, Key};
use piston_window::{Context, DrawState, UpdateArgs, Transformed, Polygon};
use piston_window::draw_state::Blend;
use crate::app::HeldKeys;
use crate::entity::{Entity, Player};
use crate::room::Room;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const DISPLAY_WIDTH_HALF: i64 = DISPLAY_WIDTH as i64 / 2;
const DISPLAY_HEIGHT_HALF: i64 = DISPLAY_HEIGHT as i64 / 2;

const TEXTURE: &[u8] = include_bytes!("../bin/spritesheet.png");

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

impl Direction {
    pub fn from(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            North => (x, y - 1),
            West => (x - 1, y),
            South => (x, y + 1),
            East => (x + 1, y),
        }
    }
}

struct Light {
    points: Vec<[f64; 2]>,
    color: [f32; 4],
}

pub struct GameView {
    texture: GlTexture,
    player: Player,
    room: Room,
    entities: Vec<Entity>,
}

impl GameView {
    pub fn new() -> Self {
        let (room, player, entities) = Room::new();
        GameView {
            texture: load_texture(),
            player,
            room,
            entities,
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

    fn render_lights(&self, gl: &mut GlGraphics, context: &Context) {
        let draw_type = DrawState::default().blend(Blend::Add);
        let mut lights = Vec::new();
        let px = 10.5;
        let py = 13.5;
        lights.push(
            Light { points: vec![[0., (py - px) * 16.], [16. * px, 16. * py], [0., 16. * (py + px)]],
                color: [0.0, 0.0, 1.0, 1.0] });

        let px = 10.5;
        let py = 17.5;
        lights.push(
            Light { points: vec![[0., (py - px) * 16.], [16. * px, 16. * py], [0., 16. * (py + px)]],
                    color: [0.0, 1.0, 0.0, 1.0] });

        let rx = 23. * 16.;
        let px = 12.5;
        let py = 13.5;
        lights.push(
            Light { points: vec![[rx, (py - px) * 16.], [16. * px, 16. * py], [rx, 16. * (py + px)]],
                    color: [1.0, 0.0, 0.0, 1.0] });

        let px = 12.5;
        let py = 17.5;
        lights.push(
            Light { points: vec![[rx, (py - px) * 16.], [16. * px, 16. * py], [rx, 16. * (py + px)]],
                    color: [0.0, 0.0, 1.0, 1.0] });

        for light in &lights {
            let polygon = Polygon::new([0.3, 0.3, 0.3, 1.]);
            polygon.draw(
                &light.points,
                &DrawState::default(),
                context.transform,
                gl,
            );
        }
        for light in &lights {
            let polygon = Polygon::new(light.color);
            polygon.draw(
                &light.points,
                &draw_type,
                context.transform,
                gl,
            );
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        // Camera
        let context = self.camera_context();

        // Lights
        self.render_lights(gl, &context);

        // Action
        self.room.render(
            &self.texture,
            &DrawState::default().blend(Blend::Multiply),
            &context,
            gl,
        );

        for entity in &self.entities {
            entity.sprite().draw(
                &self.texture,
                &DrawState::default().blend(Blend::Multiply),
                context.transform,
                gl,
            );
        }

        self.player.sprite().draw(
            &self.texture,
            &DrawState::default().blend(Blend::Multiply),
            context.transform,
            gl,
        );
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &HeldKeys) {
        self.player.update(args);
        for entity in self.entities.iter_mut() {
            entity.update(args);
        }
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
                let (nx, ny) = direction.from(self.player.x, self.player.y);
                if self.player.can_walk() && self.tile_is_passable(nx, ny) {
                    if let Some(entity_id) = self.entity_at(nx, ny) {
                        if self.entities[entity_id].is_approachable(&direction, self) {
                            self.entities[entity_id].on_approach(&direction);
                            self.player.walk(&direction);
                        }
                    }
                    else {
                        self.player.walk(&direction);
                    }
                }
            }
        }
    }

    pub fn tile_is_passable(&self, x: i32, y: i32) -> bool {
        let tile = self.room.tile_at(x, y);
        tile.map_or(false, |tile| tile.is_passable())
    }

    pub fn entity_at(&self, x: i32, y: i32) -> Option<usize> {
        self.entities.iter()
            .position(|e| e.x() == x && e.y() == y)
    }
}
