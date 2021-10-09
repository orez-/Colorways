use std::f64::consts::SQRT_2;
use opengl_graphics::GlGraphics;
use piston_window::{Context, DrawState, Ellipse, UpdateArgs};
use crate::app::lerp;

const DISPLAY_WIDTH: f64 = 200.;
const MAX: f64 = DISPLAY_WIDTH * SQRT_2 * 2.;

pub struct CircleWipe {
    x: f64,
    y: f64,
    out: bool,
    progress: f64,
}

impl CircleWipe {
    pub fn new_in(x: f64, y: f64) -> Self {
        Self { x, y, out: false, progress: 0. }
    }

    pub fn new_out(x: f64, y: f64) -> Self {
        Self { x, y, out: true, progress: 0. }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if !self.done() {
            self.progress += args.dt * 2.;
            self.progress = self.progress.min(1.);
        }
    }

    pub fn render(&self, context: &Context, gl: &mut GlGraphics) {
        let progress = if self.out { 1. - self.progress } else { self.progress };
        let dist = lerp(MAX, 0., progress);
        let halfdist = dist / 2.;
        Ellipse::new([1., 1., 1., 1.])
            .draw(
                [self.x - halfdist, self.y - halfdist, dist, dist],
                &DrawState::new_clip(),
                context.transform,
                gl,
            );
    }

    pub fn done(&self) -> bool {
        self.progress >= 1.
    }
}
