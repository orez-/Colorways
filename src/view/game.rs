use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, UpdateArgs, Transformed};
use piston_window::draw_state::Blend;
use crate::app::{Direction, HeldKeys, Input, int_lerp};
use crate::color::Color;
use crate::entity::{Entity, Player};
use crate::room::Room;
use crate::view::Transition;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const DISPLAY_WIDTH_HALF: i64 = DISPLAY_WIDTH as i64 / 2;
const DISPLAY_HEIGHT_HALF: i64 = DISPLAY_HEIGHT as i64 / 2;

const LEVEL_COMPLETE_SRC: [f64; 4] = [128., 0., 128., 112.];
const LEVEL_COMPLETE_START_DEST: [f64; 4] = [36., -112., 128., 112.];
const LEVEL_COMPLETE_END_DEST: [f64; 4] = [36., 40., 128., 112.];

#[derive(Debug)]
pub enum GameAction {
    Stop,
    ColorChange(Color),
    Win,
    DestroyBoth(usize, usize),
}

pub enum State {
    Play,
    Win(f64),
}

pub struct GameView {
    texture: GlTexture,
    player: Player,
    room: Room,
    entities: Vec<Entity>,
    light_color: Color,
    level_id: usize,
    cursor: Option<Player>,
    state: State,
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
            cursor: None,
            state: State::Play,
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

    fn render_game(&self, gl: &mut GlGraphics) {
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

    pub fn render(&self, gl: &mut GlGraphics) {
        self.render_game(gl);

        if let State::Win(progress) = self.state {
            let abs_context = self.absolute_context();
            let dest = int_lerp(LEVEL_COMPLETE_START_DEST, LEVEL_COMPLETE_END_DEST, progress);
            Image::new()
                .src_rect(LEVEL_COMPLETE_SRC)
                .rect(dest)
                .draw(
                    &self.texture,
                    &DrawState::default(),
                    abs_context.transform,
                    gl,
                );
            if let Some(cursor) = &self.cursor {
                cursor.sprite().draw(
                    &self.texture,
                    &DrawState::default(),
                    abs_context.trans(46., 107.).transform,
                    gl,
                );
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        self.player.update(args);
        for entity in self.entities.iter_mut() {
            entity.update(args);
        }
        match &mut self.state {
            State::Play => self.update_play(args, held_keys),
            State::Win(progress) => {
                if *progress < 1. {
                    *progress = *progress + args.dt * 5.;
                    if *progress >= 1. {
                        *progress = 1.;
                        self.cursor = Some(Player::new(0, 0));
                    }
                }
                if let Some(cursor) = &mut self.cursor {
                    cursor.update(args);
                    for input in held_keys.inputs() {
                        if !cursor.can_walk() { break; }
                        match input {
                            Input::Navigate(direction @ Direction::North) if cursor.y == 1 => {
                                cursor.walk(&direction);
                            },
                            Input::Navigate(direction @ Direction::South) if cursor.y == 0 => {
                                cursor.walk(&direction);
                            },
                            Input::Accept => {
                                match cursor.y {
                                    0 => return Some(Transition::Game(self.level_id + 1)),
                                    1 => return Some(Transition::Menu(self.level_id)),
                                    _ => (),
                                }
                            },
                            _ => (),
                        }
                    }
                }
                None
            }
        }
    }

    fn update_play(&mut self, _args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        let mut action = None;  // TODO: should probably be a vec? eh.
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction) => {
                    self.player.face(&direction);
                    let (nx, ny) = direction.from(self.player.x, self.player.y);
                    if self.player.can_walk() && self.tile_is_passable(nx, ny) {
                        if let Some(entity_id) = self.entity_id_at(nx, ny) {
                            if let Some(approach_action) = self.entities[entity_id].is_approachable(&direction, self) {
                                if matches!(approach_action, GameAction::Stop) { continue; }
                                if let GameAction::DestroyBoth(idx1, _) = approach_action {
                                    action = Some(GameAction::DestroyBoth(idx1, entity_id));
                                    continue;
                                }
                            }
                            // borrow checker shenanigans
                            match &self.entities[entity_id] {
                                Entity::Block(block)
                                if self.tile_in_light(block.x, block.y, &block.color) => (),
                                _ => { action = self.entities[entity_id].on_approach(&direction); },
                            }
                        }
                        self.player.walk(&direction);
                    }
                },
                Input::Reject => { return Some(Transition::Menu(self.level_id)); },
                _ => (),
            }
        }
        if let Some(action) = action {
            match action {
                GameAction::ColorChange(color) => { self.set_light_color(color); },
                GameAction::Win => {
                    self.state = State::Win(0.);
                    return Some(Transition::Win(self.level_id));
                },
                GameAction::DestroyBoth(idx1, idx2) => {
                    let mut idx = 0;
                    self.entities.retain(|_| { let m = idx1 != idx && idx2 != idx; idx += 1; m });
                },
                GameAction::Stop => (),
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
        if color == &Color::White { return true; }
        color == &self.light_color && self.room.tile_in_light(x, y, color)
    }

    pub fn entity_id_at(&self, x: i32, y: i32) -> Option<usize> {
        self.entities.iter()
            .position(|e| e.x() == x && e.y() == y)
    }

    pub fn entity_at(&self, x: i32, y: i32) -> Option<&Entity> {
        let idx = self.entity_id_at(x, y)?;
        Some(&self.entities[idx])
    }
}
