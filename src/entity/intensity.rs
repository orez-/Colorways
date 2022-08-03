use piston_window::UpdateArgs;
use std::cmp::Ordering::*;

const INTENSITY_SPEED: f32 = 2.5;

pub struct Intensity {
    pub(super) val: f32,
    pub(super) goal: f32,
}

impl Intensity {
    pub fn new(initial: f32) -> Self {
        Intensity {
            val: initial,
            goal: initial,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.val.partial_cmp(&self.goal).unwrap() {
            Equal => (),
            Less => {
                let dt = INTENSITY_SPEED * args.dt as f32;
                self.val = self.goal.min(self.val + dt);
            }
            Greater => {
                let dt = INTENSITY_SPEED * args.dt as f32;
                self.val = self.goal.max(self.val - dt);
            }
        }
    }
}
