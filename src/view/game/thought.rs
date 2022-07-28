use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, UpdateArgs};

const MAIN_THOUGHT: [f64; 4] = [80., 80., 32., 32.];
const SMALL_THOUGHT: [f64; 4] = [112., 80., 16., 16.];
const MED_THOUGHT: [f64; 4] = [112., 96., 16., 16.];

const MAIN_X: f64 = 0.;
const MAIN_Y: f64 = -34.;


enum State {
    Opening(f64),
    Idle(f64),
    Closing(f64),
}
use State::*;

pub struct Thought {
    state: State
}

impl Thought {
    pub fn new() -> Self {
        Thought { state: State::Closing(0.) }
    }

    pub fn think(&mut self) {
        if matches!(self.state, State::Closing(_)) {
            self.state = Opening(0.);
        }
    }

    pub fn dismiss(&mut self) {
        if !matches!(self.state, State::Closing(_)) {
            self.state = Closing(0.)
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match &mut self.state {
            Opening(p) => {
                *p += args.dt * 3.;
                if *p > 1. { self.state = Idle(0.); }
            }
            _ => (),
        }
    }

    pub fn render(&self,
              texture: &GlTexture,
              draw_state: &DrawState,
              context: &Context,
              gl: &mut GlGraphics) {
        if matches!(self.state, Closing(_)) { return; }

        // small
        match self.state {
            Opening(_) | Idle(_) => {
                Image::new()
                    .src_rect(SMALL_THOUGHT)
                    .rect([-8., -12., 16., 16.])
                    .draw(
                        texture,
                        draw_state,
                        context.transform,
                        gl,
                    )
            },
            Closing(_) => (),
        }

        // med
        let maybe_img = match self.state {
            Opening(p) if p < 1./6. => None,
            Opening(p) if p < 2./6. =>
                Some(Image::new()
                    .src_rect(SMALL_THOUGHT)
                    .rect([-6., -24., 16., 16.])),
            Opening(_) | Idle(_) =>
                Some(Image::new()
                    .src_rect(MED_THOUGHT)
                    .rect([-6., -24., 16., 16.])),
            Closing(_) => None,
        };
        if let Some(img) = maybe_img {
            img.draw(
                texture,
                draw_state,
                context.transform,
                gl,
            );
        }

        // big
        let maybe_img = match self.state {
            Opening(p) if p < 3./6. => None,
            Opening(p) if p < 4./6. =>
                Some(Image::new()
                    .src_rect(SMALL_THOUGHT)
                    .rect([MAIN_X, MAIN_Y, 16., 16.])),
            Opening(p) if p < 5./6. =>
                Some(Image::new()
                    .src_rect(MED_THOUGHT)
                    .rect([MAIN_X, MAIN_Y, 16., 16.])),
            Opening(_) | Idle(_) =>
                Some(Image::new()
                    .src_rect(MAIN_THOUGHT)
                    .rect([MAIN_X - 8., MAIN_Y - 8., 32., 32.])),
            Closing(_) => None,
        };
        if let Some(img) = maybe_img {
            img.draw(
                texture,
                draw_state,
                context.transform,
                gl,
            );
        }
    }
}
