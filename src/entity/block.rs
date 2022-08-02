use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::color::Color;
use crate::entity::{Entity, IEntity};
use crate::scene::{HeadlessScene, GameAction};

const TILE_SIZE: f64 = 16.;
const BLOCK_WIDTH: f64 = TILE_SIZE;
const BLOCK_HEIGHT: f64 = TILE_SIZE;
const BLOCK_OFFSET_Y: f64 = BLOCK_HEIGHT - TILE_SIZE;
const BLOCK: [f64; 4] = [0., 64., BLOCK_WIDTH, BLOCK_HEIGHT];
const PUSH_SPEED: f64 = 5.;

enum State {
    Idle,
    Slide(f64),
    // SlideSink(f64),
}
use State::*;

pub struct Block {
    pub x: i32,
    pub y: i32,
    state: State,
    facing: Direction,
    pub color: Color,
    dead: bool,
}

impl Block {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self {
            x, y, state: Idle,
            facing: Direction::East,
            color,
            dead: false,
        }
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

    pub fn push(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y -= 1,
            Direction::West => self.x -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
        }
        self.state = State::Slide(0.);
        self.facing = direction;
    }
}

impl IEntity for Block {
    fn sprite(&self) -> Image {
        let x = self.x as f64 * TILE_SIZE;
        let y = self.y as f64 * TILE_SIZE - BLOCK_OFFSET_Y;
        let (sx, sy) = self.sub_position();
        Image::new_color(self.color.as_component())
            .src_rect(BLOCK)
            .rect([x - sx, y - sy, BLOCK_WIDTH, BLOCK_HEIGHT])
    }

    fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            State::Slide(p) => {
                let new_p = p + args.dt * PUSH_SPEED;
                self.state = if new_p >= 1. { State::Idle }
                    else { State::Slide(new_p) };
            },
            // State::SlideSink(p) => {
            //     let new_p = p + args.dt * PUSH_SPEED;
            //     if new_p >= 1. {
            //         self.dead = true;
            //         return;
            //     }
            //     self.state = State::SlideSink(new_p);
            // },
            _ => ()
        }
    }

    fn on_approach(&self, entity_id: usize, direction: Direction, scene: &HeadlessScene) -> GameAction {
        if scene.tile_in_light(self.x, self.y, self.color) { return GameAction::Walk; }
        let (nx, ny) = direction.from(self.x, self.y);
        if !scene.tile_is_passable(nx, ny) { return GameAction::Stop; }
        match scene.entity_at(nx, ny) {
            Some(Entity::Water(_)) => {
                let water_id = scene.entity_id_at(nx, ny).unwrap();
                GameAction::Sink(entity_id, water_id, self.color)
            }
            Some(_) => GameAction::Stop,
            None => GameAction::Push(entity_id),
        }
    }

    fn is_dead(&self) -> bool {
        self.dead
    }
}
