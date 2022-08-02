use piston_window::{Image, UpdateArgs};
use crate::app::Direction;
use crate::scene::{HeadlessScene, GameAction};
use crate::entity;

// The boilerplatenest file. Try to stay out of here as much as possible.
pub trait IEntity {
    fn sprite(&self) -> Image;
    fn update(&mut self, _args: &UpdateArgs) { }
    fn on_approach(&self, entity_id: usize, direction: Direction, scene: &HeadlessScene) -> GameAction;
    fn is_dead(&self) -> bool { false }
}

pub enum Entity {
    Block(entity::Block),
    Exit(entity::Exit),
    Lightbulb(entity::Lightbulb),
    LightRadio(entity::LightRadio),
    LightToggle(entity::LightToggle),
    Water(entity::Water),
}
use Entity::*;

impl Entity {
    pub fn x(&self) -> i32 {
        match self {
            Block(e) => e.x,
            Exit(e) => e.x,
            Lightbulb(e) => e.x,
            LightRadio(e) => e.x,
            LightToggle(e) => e.x,
            Water(e) => e.x,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Block(e) => e.y,
            Exit(e) => e.y,
            Lightbulb(e) => e.y,
            LightRadio(e) => e.y,
            LightToggle(e) => e.y,
            Water(e) => e.y,
        }
    }
}

impl IEntity for Entity {
    fn sprite(&self) -> Image {
        match self {
            Block(e) => e.sprite(),
            Exit(e) => e.sprite(),
            Lightbulb(e) => e.sprite(),
            LightRadio(e) => e.sprite(),
            LightToggle(e) => e.sprite(),
            Water(e) => e.sprite(),
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        match self {
            Block(e) => e.update(args),
            Exit(e) => e.update(args),
            Lightbulb(e) => e.update(args),
            LightRadio(e) => e.update(args),
            LightToggle(e) => e.update(args),
            Water(e) => e.update(args),
        }
    }

    fn on_approach(&self, entity_id: usize, direction: Direction, scene: &HeadlessScene) -> GameAction {
        match self {
            Block(e) => e.on_approach(entity_id, direction, scene),
            Exit(e) => e.on_approach(entity_id, direction, scene),
            Lightbulb(e) => e.on_approach(entity_id, direction, scene),
            LightRadio(e) => e.on_approach(entity_id, direction, scene),
            LightToggle(e) => e.on_approach(entity_id, direction, scene),
            Water(e) => e.on_approach(entity_id, direction, scene),
        }
    }

    fn is_dead(&self) -> bool {
        match self {
            Block(e) => e.is_dead(),
            Exit(e) => e.is_dead(),
            Lightbulb(e) => e.is_dead(),
            LightRadio(e) => e.is_dead(),
            LightToggle(e) => e.is_dead(),
            Water(e) => e.is_dead(),
        }
    }
}
