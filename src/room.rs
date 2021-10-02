use piston_window::DrawState;
use piston_window::Context;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::Image;

const LEVEL2: &[u8] = include_bytes!("../bin/levels/level2.skb");
const TILE_SIZE: f64 = 16.;
const WALL: [f64; 4] = [32., 0., TILE_SIZE, TILE_SIZE];
const FLOOR: [f64; 4] = [32., 16., TILE_SIZE, TILE_SIZE];

enum Tile {
    Floor,
    Wall,
}
impl Tile {
    fn from_chr(chr: char) -> Self {
        match chr {
            '#' => Wall,
            _ => Floor,
        }
    }

    pub fn sprite(&self, x: usize, y: usize) -> Image {
        let src = match self {
            Wall => WALL,
            Floor => FLOOR,
        };
        Image::new()
            .src_rect(src)
            .rect([x as f64 * TILE_SIZE, y as f64 * TILE_SIZE, TILE_SIZE, TILE_SIZE])
    }
}
use Tile::*;

pub struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Room {
    pub fn new() -> Self {
        Room::from_file(LEVEL2)
    }

    pub fn from_file(bytes: &[u8]) -> Self {
        let width = bytes.iter().position(|&c| c == '\n' as u8).unwrap();
        let tiles: Vec<_> = bytes.iter()
            .filter_map(|&c| {
                let c = c as char;
                (c != '\n').then(|| Tile::from_chr(c as char))
            }).collect();
        let height = tiles.len() / width;
        Room { width, height, tiles }
    }

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

    pub fn pixel_width(&self) -> i64 {
        self.width as i64 * 16
    }

    pub fn pixel_height(&self) -> i64 {
        self.height as i64 * 16
    }
}
