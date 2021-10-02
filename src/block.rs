use piston_window::{Image, UpdateArgs};
use crate::view::{Color, Direction};

const TILE_SIZE: f64 = 16.;
const BLOCK_OFFSET_Y: f64 = 8.;
const BLOCK_WIDTH: f64 = 16.;
const BLOCK_HEIGHT: f64 = 24.;
const BLOCK: [f64; 4] = [64., 0., BLOCK_WIDTH, BLOCK_HEIGHT];

const GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.];
const RED: [f32; 4] = [1., 0., 0., 1.];
const WHITE: [f32; 4] = [1., 1., 1., 1.];

enum State {
    Idle,
    Slide(f64),
}
use State::*;

pub struct Block {
    pub x: i32,
    pub y: i32,
    state: State,
    facing: Direction,
    color: Color,
}

impl Block {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Block {
            x, y, state: Idle,
            facing: Direction::East,
            color,
        }
    }

    fn color(&self) -> [f32; 4] {
        match self.color {
            Color::Gray => GRAY,
            Color::Red => RED,
            Color::White => WHITE,
            _ => WHITE,
        }
    }

    pub fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE - BLOCK_OFFSET_Y;
        let (sx, sy) = self.sub_position();
        Image::new_color(self.color())
            .src_rect(BLOCK)
            .rect([x - sx, y - sy, BLOCK_WIDTH, BLOCK_HEIGHT])
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            State::Slide(p) => {
                let new_p = p + args.dt * 5.;
                self.state = if new_p >= 1. { State::Idle }
                    else { State::Slide(new_p) };
            },
            _ => ()
        };
    }

    fn sub_position(&self) -> (f64, f64) {
        if let State::Slide(p) = self.state {
            let progress = ((1. - p) * TILE_SIZE) as i8 as f64;
            return match self.facing {
                Direction::North => (0., -progress),
                Direction::East => (progress, 0.),
                Direction::South => (0., progress),
                Direction::West => (-progress, 0.),
            }
        }
        (0., 0.)
    }

    pub fn push(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.y -= 1,
            Direction::West => self.x -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
        }
        self.state = State::Slide(0.);
        self.facing = direction.clone();
    }
}
