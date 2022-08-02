use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::app::Direction::*;

// fn flip(coords: [f64; 4]) -> [f64; 4] {
//     [coords[0] + coords[2], coords[1], -coords[2], coords[3]]
// }
const TILE_SIZE: f64 = 16.;
const PLAYER_WIDTH: f64 = 16.;
const PLAYER_HEIGHT: f64 = 16.;
const PLAYER_WIDTH_HALF: f64 = PLAYER_WIDTH / 2.;
const PLAYER_HEIGHT_HALF: f64 = PLAYER_HEIGHT / 2.;
const PLAYER_IDLE_RIGHT: [f64; 4] = [0., 0., PLAYER_WIDTH, PLAYER_HEIGHT];
const PLAYER_IDLE_LEFT: [f64; 4] = [16., 0., -PLAYER_WIDTH, PLAYER_HEIGHT];
const PLAYER_RUN_RIGHT: [f64; 4] = [0., 16., PLAYER_WIDTH, PLAYER_HEIGHT];
const PLAYER_RUN_LEFT: [f64; 4] = [16., 16., -PLAYER_WIDTH, PLAYER_HEIGHT];

enum State {
    Idle,
    Walk(f64),
}

pub struct Player {
    face_left: bool,
    facing: Direction,
    pub x: i32,
    pub y: i32,
    state: State,
    step_x: f64,
    step_y: f64,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            face_left: false,
            facing: West,
            x, y,
            state: State::Idle,
            step_x: TILE_SIZE, step_y: TILE_SIZE,
        }
    }

    pub fn new_cursor(x: i32, y: i32, step_x: f64, step_y: f64) -> Self {
        Player {
            face_left: false,
            facing: East,
            x, y,
            state: State::Idle,
            step_x, step_y,
        }
    }

    fn sub_position(&self) -> (f64, f64) {
        if let State::Walk(p) = self.state {
            let dx = ((1. - p) * self.step_x) as i8 as f64;
            let dy = ((1. - p) * self.step_y) as i8 as f64;
            return match self.facing {
                North => (0., -dy),
                East => (dx, 0.),
                South => (0., dy),
                West => (-dx, 0.),
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

    fn sprite_src(&self) -> [f64; 4] {
        match self.state {
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
        }
    }

    pub fn pixel_coord(&self) -> (f64, f64) {
        let (sx, sy) = self.sub_position();
        let x = self.x as f64 * self.step_x;
        let y = self.y as f64 * self.step_y;
        (x - sx, y - sy)
    }

    pub fn sprite(&self) -> Image {
        let src = self.sprite_src();
        let (x, y) = self.pixel_coord();
        Image::new()
            .src_rect(src)
            .rect([x, y, PLAYER_WIDTH, PLAYER_HEIGHT])
    }

    pub fn center(&self) -> (i64, i64) {
        let (sx, sy) = self.sub_position();
        let x = self.x as f64 * self.step_x + PLAYER_WIDTH_HALF;
        let y = self.y as f64 * self.step_y + PLAYER_HEIGHT_HALF;
        (x as i64 - sx as i64, y as i64 - sy as i64)
    }

    pub fn face(&mut self, direction: Direction) {
        match direction {
            West => self.face_left = true,
            East => self.face_left = false,
            _ => (),
        };
    }

    pub fn walk(&mut self, direction: Direction) -> bool {
        match self.state {
            State::Idle => {
                match direction {
                    North => self.y -= 1,
                    West => self.x -= 1,
                    South => self.y += 1,
                    East => self.x += 1,
                }
                self.facing = direction;
                self.state = State::Walk(0.);
                true
            }
            State::Walk(_) => false,
        }
    }

    pub fn can_walk(&self) -> bool {
        matches!(&self.state, State::Idle)
    }

    pub fn undo(&mut self, direction: Direction) {
        let (nx, ny) = direction.from(self.x, self.y);
        self.x = nx;
        self.y = ny;
        match direction {
            Direction::West => { self.face_left = false; }
            Direction::East => { self.face_left = true; }
            _ => (),
        }
    }

    #[cfg(test)]
    pub fn resolve(&mut self) {
        self.state = State::Idle;
    }
}
