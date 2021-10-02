use piston_window::{Image, UpdateArgs};
use crate::view::Direction;
use crate::entity;

// The boilerplatenest file. Try to stay out of here as much as possible.
pub enum Entity {
    Block(entity::Block),
    Lightbulb(entity::Lightbulb),
}
use Entity::*;

impl Entity {
    pub fn sprite(&self) -> Image {
        match self {
            Block(e) => e.sprite(),
            Lightbulb(e) => e.sprite(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self {
            Block(e) => e.update(args),
            Lightbulb(e) => e.update(args),
        }
    }

    pub fn push(&mut self, direction: &Direction) {
        match self {
            Block(e) => e.push(direction),
            Lightbulb(e) => e.push(direction),
        }
    }

    pub fn x(&self) -> i32 {
        match self {
            Block(e) => e.x,
            Lightbulb(e) => e.x,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Block(e) => e.y,
            Lightbulb(e) => e.y,
        }
    }
}
