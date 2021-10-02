use piston_window::{Image, UpdateArgs};
use crate::view::Direction;
use crate::view::Direction::*;

// fn flip(coords: [f64; 4]) -> [f64; 4] {
//     [coords[0] + coords[2], coords[1], -coords[2], coords[3]]
// }
const TILE_SIZE: f64 = 16.;
const PLAYER_WIDTH: f64 = 16.;
const PLAYER_HEIGHT: f64 = 24.;
const PLAYER_WIDTH_HALF: f64 = PLAYER_WIDTH / 2.;
const PLAYER_HEIGHT_HALF: f64 = PLAYER_HEIGHT / 2.;
const PLAYER_IDLE_RIGHT: [f64; 4] = [0., 0., PLAYER_WIDTH, PLAYER_HEIGHT];
const PLAYER_IDLE_LEFT: [f64; 4] = [16., 0., -PLAYER_WIDTH, PLAYER_HEIGHT];
const PLAYER_RUN_RIGHT: [f64; 4] = [0., 24., PLAYER_WIDTH, PLAYER_HEIGHT];
const PLAYER_RUN_LEFT: [f64; 4] = [16., 24., -PLAYER_WIDTH, PLAYER_HEIGHT];

enum State {
    Idle,
    Walk(f64),
}

pub struct Player {
    face_left: bool,
    facing: Direction,
    x: i32,
    y: i32,
    state: State,
}

impl Player {
    pub fn new() -> Self {
        Player {
            face_left: false,
            facing: West,
            x: 0,
            y: 0,
            state: State::Idle,
        }
    }

    fn sub_position(&self) -> (f64, f64) {
        if let State::Walk(p) = self.state {
            let progress = ((1. - p) * TILE_SIZE) as i8 as f64;
            return match self.facing {
                North => (0., -progress),
                East => (progress, 0.),
                South => (0., progress),
                West => (-progress, 0.),
            }
        }
        (0., 0.)
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            State::Walk(p) => {
                let new_p = p + args.dt * 5.;
                self.state = if new_p >= 1. { State::Idle }
                    else { State::Walk(new_p) };
            },
            _ => ()
        };
    }

    pub fn sprite(&self) -> Image {
        let src = match self.state {
            State::Walk(p) => {
                if p % 0.5 <= 0.25 {
                    if self.face_left { PLAYER_RUN_LEFT }
                    else { PLAYER_RUN_RIGHT }
                }
                else {
                    if self.face_left { PLAYER_IDLE_LEFT }
                    else { PLAYER_IDLE_RIGHT }
                }
            }
            State::Idle => {
                if self.face_left { PLAYER_IDLE_LEFT }
                else { PLAYER_IDLE_RIGHT }
            }
        };
        let (sx, sy) = self.sub_position();
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE;
        Image::new()
            .src_rect(src)
            .rect([x - sx, y - sy, PLAYER_WIDTH, PLAYER_HEIGHT])
    }

    pub fn center(&self) -> (i64, i64) {
        let (sx, sy) = self.sub_position();
        let x = self.x as f64 * TILE_SIZE + PLAYER_WIDTH_HALF;
        let y = self.y as f64 * TILE_SIZE + PLAYER_HEIGHT_HALF;
        (x as i64 - sx as i64, y as i64 - sy as i64)
    }

    pub fn face(&mut self, direction: &Direction) {
        match direction {
            West => self.face_left = true,
            East => self.face_left = false,
            _ => (),
        };
    }

    pub fn walk(&mut self, direction: &Direction) -> bool {
        match self.state {
            State::Idle => {
                match direction {
                    North => self.y -= 1,
                    West => self.x -= 1,
                    South => self.y += 1,
                    East => self.x += 1,
                }
                self.facing = direction.clone();
                self.state = State::Walk(0.);
                true
            }
            State::Walk(_) => false,
        }
    }
}
