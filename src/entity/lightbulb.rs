use piston_window::Graphics;
use piston_window::{Context, DrawState, Image, Polygon, UpdateArgs};
use piston_window::draw_state::Blend;
use crate::app::{Direction, lerpn};
use crate::color::Color;
use crate::entity::IEntity;
use crate::scene::{Scene, GameAction};

type Rect = [f64; 4];
type GlColor = [f32; 4];

const TILE_SIZE: f64 = 16.;
const LIGHTBULB_ON: Rect = [16., 16., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_RISING_1: Rect = [16., 48., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_RISING_2: Rect = [16., 32., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_FALLING_1: Rect = [32., 32., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_FALLING_2: Rect = [32., 48., TILE_SIZE, TILE_SIZE];
const LIGHTBULB_OFF: Rect = [16., 64., TILE_SIZE, TILE_SIZE];
const LIGHT_BASE: GlColor = [0.3, 0.3, 0.3, 1.0];

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
    pub fn new(x: i32, y: i32, color: Color, light_polygon: Vec<[f64; 2]>) -> Self {
        Self { x, y, color, state: State::Off, light_polygon }
    }

    pub fn draw_light(&self, state: &DrawState, context: &Context, gl: &mut impl Graphics) {
        let mut color = self.color.as_component();
        color[3] = self.light_intensity();
        self.draw_light_fan(
            color,
            &state.blend(Blend::Lighter),
            context,
            gl,
        );
    }

    pub fn draw_light_base(&self, ambient_color: GlColor, state: &DrawState, context: &Context, gl: &mut impl Graphics) {
        let color = lerpn(ambient_color, LIGHT_BASE, self.light_intensity());
        self.draw_light_fan(
            color,
            // &state.blend(Blend::Invert),
            &state,
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

    pub fn toggle(&mut self) {
        match self.state {
            State::On | State::Rising(_) => self.turn_off(),
            State::Off | State::Falling(_) => self.turn_on(),
        }
    }

    /// Value from 0 to 1 representing light intensity
    fn light_intensity(&self) -> f32 {
        match self.state {
            State::On => 1.,
            State::Rising(p) => { p as f32 },
            State::Off => 0.,
            State::Falling(p) => { 1. - p as f32 },
        }
    }

    fn draw_light_fan(&self, color: GlColor, state: &DrawState, context: &Context, gl: &mut impl Graphics) {
        if matches!(self.state, State::Off) { return; }
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

impl IEntity for Lightbulb {
    fn sprite(&self) -> Image {
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

    fn update(&mut self, args: &UpdateArgs) {
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

    fn on_approach(&self, _entity_id: usize, _direction: Direction, _game: &Scene) -> GameAction {
        GameAction::Stop
    }
}
