use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::view::{GameAction, GameView};
use crate::entity;

// The boilerplatenest file. Try to stay out of here as much as possible.
pub enum Entity {
    Block(entity::Block),
    Exit(entity::Exit),
    Lightbulb(entity::Lightbulb),
    LightSwitch(entity::LightSwitch),
    Water(entity::Water),
}
use Entity::*;

impl Entity {
    pub fn sprite(&self) -> Image {
        match self {
            Block(e) => e.sprite(),
            Exit(e) => e.sprite(),
            Lightbulb(e) => e.sprite(),
            LightSwitch(e) => e.sprite(),
            Water(e) => e.sprite(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self {
            Block(e) => e.update(args),
            Exit(e) => e.update(args),
            Lightbulb(e) => e.update(args),
            LightSwitch(e) => e.update(args),
            Water(e) => e.update(args),
        }
    }

    pub fn on_approach(&self, entity_id: usize, direction: &Direction, view: &GameView) -> GameAction {
        match self {
            Block(e) => e.on_approach(entity_id, direction, view),
            Exit(e) => e.on_approach(entity_id, direction, view),
            Lightbulb(e) => e.on_approach(entity_id, direction, view),
            LightSwitch(e) => e.on_approach(entity_id, direction, view),
            Water(e) => e.on_approach(entity_id, direction, view),
        }
    }

    pub fn x(&self) -> i32 {
        match self {
            Block(e) => e.x,
            Exit(e) => e.x,
            Lightbulb(e) => e.x,
            LightSwitch(e) => e.x,
            Water(e) => e.x,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Block(e) => e.y,
            Exit(e) => e.y,
            Lightbulb(e) => e.y,
            LightSwitch(e) => e.y,
            Water(e) => e.y,
        }
    }
}
