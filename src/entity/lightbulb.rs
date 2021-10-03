use opengl_graphics::GlGraphics;
use piston_window::{Context, DrawState, Image, Polygon, UpdateArgs};
use crate::color::Color;
use crate::room::Room;
use crate::line_of_sight::{Visibility, line_of_sight};
use crate::view::{Direction, GameView};

const TILE_SIZE: f64 = 16.;
const LIGHTBULB: [f64; 4] = [16., 16., TILE_SIZE, TILE_SIZE];

pub struct Lightbulb {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    light_polygon: Vec<[f64; 2]>,
}

impl Lightbulb {
    pub fn new(x: i32, y: i32, color: Color, room: &Room) -> Self {
        let Visibility { polygon_pts, .. } = line_of_sight(x, y, room);
        Self { x, y, color, light_polygon: polygon_pts }
    }

    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(LIGHTBULB)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn is_approachable(&self, _direction: &Direction, _game: &GameView) -> bool {
        false
    }
    pub fn on_approach(&mut self, _direction: &Direction) { }

    pub fn draw_light(&self, color: [f32; 4], state: &DrawState, context: &Context, gl: &mut GlGraphics) {
        // Need to triangulate the polygon: opengl doesn't draw concave polygons.
        // Fortunately we axiomatically have a point that can see all vertexes: the sprite center.
        // TODO: look into how to accomplish a "fan"
        let center = [(self.x as f64 + 0.5) * TILE_SIZE, (self.y as f64 + 0.5) * TILE_SIZE];
        let polygon = Polygon::new(color);
        for v in self.light_polygon.windows(2) {
            polygon.draw(
                &[v[0], v[1], center],
                state,
                context.transform,
                gl,
            );
        }
    }
}
