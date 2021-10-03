use piston_window::{Image, UpdateArgs};
use crate::view::{Direction, GameAction, GameView};
use crate::entity;

// The boilerplatenest file. Try to stay out of here as much as possible.
pub enum Entity {
    Block(entity::Block),
    Lightbulb(entity::Lightbulb),
    LightSwitch(entity::LightSwitch),
}
use Entity::*;

impl Entity {
    pub fn sprite(&self) -> Image {
        match self {
            Block(e) => e.sprite(),
            Lightbulb(e) => e.sprite(),
            LightSwitch(e) => e.sprite(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self {
            Block(e) => e.update(args),
            Lightbulb(e) => e.update(args),
            LightSwitch(e) => e.update(args),
        }
    }

    pub fn is_approachable(&self, direction: &Direction, game: &GameView) -> bool {
        match self {
            Block(e) => e.is_approachable(direction, game),
            Lightbulb(e) => e.is_approachable(direction, game),
            LightSwitch(e) => e.is_approachable(direction, game),
        }
    }

    pub fn on_approach(&mut self, direction: &Direction) -> Option<GameAction> {
        match self {
            Block(e) => e.on_approach(direction),
            Lightbulb(e) => e.on_approach(direction),
            LightSwitch(e) => e.on_approach(direction),
        }
    }

    pub fn x(&self) -> i32 {
        match self {
            Block(e) => e.x,
            Lightbulb(e) => e.x,
            LightSwitch(e) => e.x,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Block(e) => e.y,
            Lightbulb(e) => e.y,
            LightSwitch(e) => e.y,
        }
    }
}
