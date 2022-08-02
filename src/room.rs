use piston_window::{Context, DrawState, Image};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use crate::color::Color;

const TILE_SIZE: f64 = 16.;
const WALL: [f64; 4] = [32., 0., TILE_SIZE, TILE_SIZE];
const FLOOR: [f64; 4] = [32., 16., TILE_SIZE, TILE_SIZE];

#[derive(Clone, Copy)]
pub enum Tile {
    Floor,
    Wall,
}
use Tile::*;

impl Tile {
    pub fn sprite(&self, x: usize, y: usize) -> Image {
        let src = match self {
            Wall => WALL,
            Floor => FLOOR,
        };
        Image::new()
            .src_rect(src)
            .rect([x as f64 * TILE_SIZE, y as f64 * TILE_SIZE, TILE_SIZE, TILE_SIZE])
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Wall => false,
            Floor => true,
        }
    }

    pub fn is_transparent(&self) -> bool {
        // TODO: ᖍ(∙⟞∙)ᖌ
        self.is_passable()
    }
}

pub struct Room {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub sees_color: Vec<Color>,  // XXX: this is insufficient if we want to support CMYW lights
}

impl Room {
    pub fn render(&self,
                  texture: &GlTexture,
                  draw_state: &DrawState,
                  context: &Context,
                  gl: &mut GlGraphics) {
        for (i, elem) in self.tiles.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            elem.sprite(x, y)
                .draw(texture, draw_state, context.transform, gl);
        }
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 { return None; }
        let idx = self.width * (y as usize) + x as usize;
        self.tiles.get(idx).cloned()
    }

    pub fn tile_light(&self, x: i32, y: i32, light_color: Color) -> Color {
        if x < 0 || y < 0 { panic!(); }
        let idx = self.width * (y as usize) + x as usize;
        let possible_color = *self.sees_color.get(idx).unwrap_or(&Color::GRAY);
        possible_color & light_color
    }

    pub fn pixel_width(&self) -> i64 {
        self.width as i64 * 16
    }

    pub fn pixel_height(&self) -> i64 {
        self.height as i64 * 16
    }
}
