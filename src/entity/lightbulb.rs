use opengl_graphics::GlGraphics;
use piston_window::{Context, DrawState, Image, Polygon, UpdateArgs};
use piston_window::draw_state::Blend;
use crate::color::Color;
use crate::room::Room;
use crate::line_of_sight::{Visibility, line_of_sight};
use crate::view::{Direction, GameAction, GameView};

const TILE_SIZE: f64 = 16.;
const LIGHTBULB_ON: [f64; 4] = [16., 16., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_RISING_1: [f64; 4] = [16., 48., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_RISING_2: [f64; 4] = [16., 32., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_FALLING_1: [f64; 4] = [32., 32., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_FALLING_2: [f64; 4] = [32., 48., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_OFF: [f64; 4] = [16., 64., TILE_SIZE, TILE_SIZE];

fn lerp(left: [f32; 4], right: [f32; 4], p: f32) -> [f32; 4] {
    [
        (right[0] - left[0]) * p + left[0],
        (right[1] - left[1]) * p + left[1],
        (right[2] - left[2]) * p + left[2],
        (right[3] - left[3]) * p + left[3],
    ]
}

enum State {
    On,
    Rising(f64),
    Off,
    Falling(f64),
}

pub struct Lightbulb {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    state: State,
    light_polygon: Vec<[f64; 2]>,
}

impl Lightbulb {
    pub fn new(x: i32, y: i32, color: Color, room: &Room) -> Self {
        let Visibility { polygon_pts, .. } = line_of_sight(x, y, room);
        Self { x, y, color, state: State::Off, light_polygon: polygon_pts }
    }

    pub fn sprite(&self) -> Image {
        let src = match self.state {
            State::On => LIGHTBULB_ON,
            State::Off => LIGHTBULB_OFF,
            State::Rising(p) if p <= 0.2 => { LIGHTBULB_OFF },
            State::Rising(p) if p <= 0.5 => { LIGHTBULB_RISING_1 },
            State::Rising(p) if p <= 0.8 => { LIGHTBULB_RISING_2 },
            State::Rising(_) => { LIGHTBULB_ON },
            State::Falling(p) if p <= 0.2 => { LIGHTBULB_FALLING_1 },
            State::Falling(p) if p <= 0.4 => { LIGHTBULB_FALLING_2 },
            State::Falling(_) => { LIGHTBULB_OFF },
        };
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new_color(self.color.as_component())
            .src_rect(src)
            .rect([x, y, TILE_SIZE, TILE_SIZE])
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            State::On | State::Off => (),
            State::Rising(p) => {
                let p2 = p + args.dt * 5.;
                if p2 >= 1. { self.state = State::On; }
                else { self.state = State::Rising(p2); }
            },
            State::Falling(p) => {
                let p2 = p + args.dt * 5.;
                if p2 >= 1. { self.state = State::Off; }
                else { self.state = State::Falling(p2); }
            },
        }
    }

    pub fn is_approachable(&self, _direction: &Direction, _game: &GameView) -> bool {
        false
    }
    pub fn on_approach(&mut self, _direction: &Direction) -> Option<GameAction> { None }

    pub fn draw_light_base(&self, context: &Context, gl: &mut GlGraphics) {
        self.draw_light_fan(
            [0.3, 0.3, 0.3, 1.],
            &DrawState::default(),
            context,
            gl,
        );
    }

    pub fn draw_light(&self, context: &Context, gl: &mut GlGraphics) {
        self.draw_light_fan(
            self.color.as_light_component(),
            &DrawState::default().blend(Blend::Multiply),
            context,
            gl,
        );
    }

    pub fn turn_on(&mut self) {
        self.state = State::Rising(0.);
    }

    pub fn turn_off(&mut self) {
        self.state = State::Falling(0.);
    }

    fn light_alpha(&self) -> f32 {
        match self.state {
            State::On => 1.,
            State::Rising(p) => { p as f32 },
            State::Off => 0.,
            State::Falling(p) => { 1. - p as f32 },
        }
    }

    fn draw_light_fan(&self, color: [f32; 4], state: &DrawState, context: &Context, gl: &mut GlGraphics) {
        if matches!(self.state, State::Off) { return; }
        let color = lerp([1., 1., 1., 1.], color, self.light_alpha());
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
