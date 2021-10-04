use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, UpdateArgs, Transformed};
use piston_window::draw_state::Blend;
use crate::app::{HeldKeys, Input};
use crate::color::Color;
use crate::entity::{Entity, Player};
use crate::room::Room;
use crate::view::Transition;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const DISPLAY_WIDTH_HALF: i64 = DISPLAY_WIDTH as i64 / 2;
const DISPLAY_HEIGHT_HALF: i64 = DISPLAY_HEIGHT as i64 / 2;

pub enum GameAction {
    ColorChange(Color),
}

pub struct GameView {
    texture: GlTexture,
    player: Player,
    room: Room,
    entities: Vec<Entity>,
    light_color: Color,
    level_id: usize,
}

impl GameView {
    pub fn new(level_id: usize) -> Self {
        let (room, player, entities, light_color) = Room::new(level_id);
        let mut game = GameView {
            texture: crate::app::load_texture(),
            player,
            room,
            entities,
            light_color: Color::Gray,
            level_id,
        };
        game.set_light_color(light_color);
        game
    }

    fn absolute_context(&self) -> Context {
        Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    fn camera_context(&self) -> Context {
        let (x, y) = self.camera();
        let x = x as f64;
        let y = y as f64;
        self.absolute_context()
            .trans(-x + DISPLAY_WIDTH / 2., -y + DISPLAY_HEIGHT / 2.)
    }

    // TODO: if the level's too small probably center it instead
    fn camera(&self) -> (i64, i64) {
        let (x, y) = self.player.center();
        let mut xs = vec![DISPLAY_WIDTH_HALF, x, self.room.pixel_width() - DISPLAY_WIDTH_HALF];
        let mut ys = vec![DISPLAY_HEIGHT_HALF, y, self.room.pixel_height() - DISPLAY_HEIGHT_HALF];
        xs.sort();
        ys.sort();
        (xs[1], ys[1])
    }

    fn render_lights(&self, gl: &mut GlGraphics, context: &Context) {
        let lights: Vec<_> = self.entities.iter().filter_map(|e| {
            if let Entity::Lightbulb(bulb) = e { Some(bulb) }
            else { None }
        }).collect();

        // for light in &lights {
        //     light.draw_light_base(context, gl);
        // }

        for light in &lights {
            light.draw_light(context, gl);
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        // Camera
        let context = self.camera_context();

        // Action
        self.room.render(
            &self.texture,
            &DrawState::default(),
            &context,
            gl,
        );

        for entity in &self.entities {
            entity.sprite().draw(
                &self.texture,
                &DrawState::default(),
                context.transform,
                gl,
            );
        }

        self.player.sprite().draw(
            &self.texture,
            &DrawState::default(),
            context.transform,
            gl,
        );

        // Lights
        self.render_lights(gl, &context);
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.player.update(args);
        for entity in self.entities.iter_mut() {
            entity.update(args);
        }
        let mut action = None;  // TODO: should probably be a vec? eh.
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction) => {
                    self.player.face(&direction);
                    let (nx, ny) = direction.from(self.player.x, self.player.y);
                    if self.player.can_walk() && self.tile_is_passable(nx, ny) {
                        if let Some(entity_id) = self.entity_at(nx, ny) {
                            if self.entities[entity_id].is_approachable(&direction, self) {
                                // borrow checker shenanigans
                                match &self.entities[entity_id] {
                                    Entity::Block(block)
                                    if self.tile_in_light(block.x, block.y, &block.color) => (),
                                    _ => { action = self.entities[entity_id].on_approach(&direction); },
                                }
                                self.player.walk(&direction);
                            }
                        }
                        else {
                            self.player.walk(&direction);
                        }
                    }
                },
                Input::Reject => { return Some(Transition::Menu(self.level_id)); },
                _ => (),
            }
        }
        if let Some(action) = action {
            match action {
                GameAction::ColorChange(color) => { self.set_light_color(color); },
            }
        }
        None
    }

    pub fn set_light_color(&mut self, color: Color) {
        if self.light_color == color { return; }
        for entity in self.entities.iter_mut() {
            if let Entity::Lightbulb(bulb) = entity {
                if bulb.color == self.light_color { bulb.turn_off(); }
                else if bulb.color == color { bulb.turn_on(); }
            }
        }
        self.light_color = color;
    }

    pub fn tile_is_passable(&self, x: i32, y: i32) -> bool {
        let tile = self.room.tile_at(x, y);
        tile.map_or(false, |tile| tile.is_passable())
    }

    pub fn tile_in_light(&self, x: i32, y: i32, color: &Color) -> bool {
        color == &self.light_color && self.room.tile_in_light(x, y, color)
    }

    pub fn entity_at(&self, x: i32, y: i32) -> Option<usize> {
        self.entities.iter()
            .position(|e| e.x() == x && e.y() == y)
    }
}