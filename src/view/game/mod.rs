mod thought;

use crate::app::{int_lerp4, Direction, HeldKeys, Input};
use crate::circle_wipe::CircleWipe;
use crate::color::Color;
use crate::entity::{Block, Entity, IEntity, Player, Water};
use crate::room::Room;
use crate::view::game::thought::Thought;
use crate::view::Transition;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, Transformed, UpdateArgs};

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
    Walk,
    Push(usize),
    ColorChange(Color),
    Win,
    Sink(usize, usize, Color),
}

enum HistoryEventType {
    Walk,
    Push,
    Sink(Color),
    ColorChange(Color),
}

struct HistoryEvent {
    direction: Direction,
    event_type: HistoryEventType,
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
    thought: Thought,
    history: Vec<HistoryEvent>,
    fade: Option<CircleWipe>,
    staged_transition: Option<Transition>,
}

impl GameView {
    pub fn new(level_id: usize) -> Self {
        let (room, player, entities, light_color) = Room::new(level_id);
        let (cx, cy) = player.center();
        let mut game = GameView {
            texture: crate::app::load_texture(),
            player,
            room,
            entities,
            light_color: Color::Gray,
            level_id,
            cursor: None,
            state: State::Play,
            thought: Thought::new(),
            history: Vec::new(),
            fade: Some(CircleWipe::new_out(cx as f64, cy as f64)),
            staged_transition: None,
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
        let mut xs = vec![
            DISPLAY_WIDTH_HALF,
            x,
            self.room.pixel_width() - DISPLAY_WIDTH_HALF,
        ];
        let mut ys = vec![
            DISPLAY_HEIGHT_HALF,
            y,
            self.room.pixel_height() - DISPLAY_HEIGHT_HALF,
        ];
        xs.sort();
        ys.sort();
        (xs[1], ys[1])
    }

    fn render_lights(&self, gl: &mut GlGraphics, draw_state: &DrawState, context: &Context) {
        let lights: Vec<_> = self.entities.iter().filter_map(|e| {
            if let Entity::Lightbulb(bulb) = e { Some(bulb) }
            else { None }
        }).collect();

        for light in &lights {
            light.draw_light(context, draw_state, gl);
        }
    }

    fn render_game(&self, draw_state: &DrawState, gl: &mut GlGraphics) {
        // Camera
        let context = self.camera_context();

        // Action
        self.room.render(
            &self.texture,
            &draw_state,
            &context,
            gl,
        );

        for entity in &self.entities {
            entity.sprite().draw(
                &self.texture,
                &draw_state,
                context.transform,
                gl,
            );
        }

        self.player.sprite().draw(
            &self.texture,
            &draw_state,
            context.transform,
            gl,
        );

        // Lights
        self.render_lights(gl, &draw_state, &context);

        // Thoughts
        let (px, py) = self.player.pixel_coord();
        self.thought.render(
            &self.texture,
            &draw_state,
            &context.trans(px, py),
            gl,
        );
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let context = self.camera_context();
        let abs_context = self.absolute_context();
        let cursor_context = abs_context.trans(46., 107.);

        let draw_state = if let Some(fade) = &self.fade {
            let fade_context = if self.cursor.is_some() { cursor_context } else { context };
            fade.render(&fade_context, gl);
            DrawState::new_inside()
        }
        else { DrawState::default() };

        self.render_game(&draw_state, gl);

        if let State::Win(progress) = self.state {
            let dest = int_lerp4(LEVEL_COMPLETE_START_DEST, LEVEL_COMPLETE_END_DEST, progress);
            Image::new().src_rect(LEVEL_COMPLETE_SRC).rect(dest).draw(
                &self.texture,
                &draw_state,
                abs_context.transform,
                gl,
            );
            if let Some(cursor) = &self.cursor {
                cursor.sprite().draw(
                    &self.texture,
                    &draw_state,
                    cursor_context.transform,
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
        self.thought.update(args);

        if let Some(fade) = &mut self.fade {
            fade.update(args);
            if fade.done() {
                self.fade = None;
                return self.staged_transition.take();
            }
            return None;
        }

        match &mut self.state {
            State::Play => self.update_play(held_keys),
            State::Win(progress) => {
                if *progress < 1. {
                    *progress = *progress + args.dt * 5.;
                    if *progress >= 1. {
                        *progress = 1.;
                        self.cursor = Some(Player::new(0, 0));
                    }
                }
                self.update_win(args, held_keys)
            }
        }
    }

    fn update_play(&mut self, held_keys: &mut HeldKeys) -> Option<Transition> {
        for input in held_keys.inputs() {
            match input {
                Input::Navigate(direction) => {
                    self.thought.dismiss();
                    self.player.face(&direction);
                    let (nx, ny) = direction.from(self.player.x, self.player.y);
                    if self.player.can_walk() && self.tile_is_passable(nx, ny) {
                        let action = if let Some(entity_id) = self.entity_id_at(nx, ny) {
                            self.entities[entity_id].on_approach(entity_id, &direction, self)
                        }
                        else { GameAction::Walk };
                        return self.handle_action(&direction, action);
                    }
                }
                Input::Accept => { self.fade_out(Transition::Menu(self.level_id)); }
                Input::Reject => { self.undo(); }
                Input::Help => { self.thought.think(); }
            }
        }
        None
    }

    fn handle_action(&mut self, direction: &Direction, action: GameAction) -> Option<Transition> {
        if matches!(action, GameAction::Stop) { return None; }
        self.player.walk(&direction);
        match action {
            GameAction::Walk => {
                self.history.push(HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Walk,
                });
            },
            GameAction::ColorChange(color) => {
                self.history.push(HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::ColorChange(self.light_color.clone()),
                });
                self.set_light_color(color);
            }
            GameAction::Win => {
                self.state = State::Win(0.);
                return Some(Transition::Win(self.level_id));
            }
            GameAction::Sink(idx1, idx2, color) => {
                self.history.push(HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Sink(color),
                });
                let mut idx = 0;
                self.entities.retain(|_| {
                    let m = idx1 != idx && idx2 != idx;
                    idx += 1;
                    m
                });
            }
            GameAction::Push(entity_id) => {
                self.history.push(HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Push,
                });
                if let Entity::Block(block) = &mut self.entities[entity_id] {
                    block.push(direction);
                } else { unreachable!(); }
            }
            GameAction::Stop => unreachable!(),
        }
        None
    }

    fn update_win(&mut self, args: &UpdateArgs, held_keys: &mut HeldKeys) -> Option<Transition> {
        let mut transition = None;
        if let Some(cursor) = &mut self.cursor {
            cursor.update(args);
            for input in held_keys.inputs() {
                if !cursor.can_walk() {
                    break;
                }
                match input {
                    Input::Navigate(direction @ Direction::North) if cursor.y == 1 => {
                        cursor.walk(&direction);
                    }
                    Input::Navigate(direction @ Direction::South) if cursor.y == 0 => {
                        cursor.walk(&direction);
                    }
                    Input::Accept => match cursor.y {
                        0 => { transition = Some(Transition::Game(self.level_id + 1)); }
                        1 => { transition = Some(Transition::Menu(self.level_id)); }
                        _ => (),
                    },
                    _ => (),
                }
            }
        };
        if let Some(t) = transition { self.fade_out(t); }
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
        color.contains(&self.light_color) && self.room.tile_in_light(x, y, &self.light_color)
    }

    pub fn entity_id_at(&self, x: i32, y: i32) -> Option<usize> {
        self.entities.iter().position(|e| e.x() == x && e.y() == y)
    }

    pub fn entity_at(&self, x: i32, y: i32) -> Option<&Entity> {
        let idx = self.entity_id_at(x, y)?;
        Some(&self.entities[idx])
    }

    fn entity_at_mut(&mut self, x: i32, y: i32) -> Option<&mut Entity> {
        let idx = self.entity_id_at(x, y)?;
        Some(&mut self.entities[idx])
    }

    fn fade_out(&mut self, transition: Transition) {
        let (x, y) = self.cursor.as_ref().unwrap_or(&self.player).center();
        self.fade = Some(CircleWipe::new_in(x as f64, y as f64));
        self.staged_transition = Some(transition);
    }

    fn undo(&mut self) {
        let event = match self.history.pop() {
            Some(value) => value,
            None => return,
        };
        let px = self.player.x;
        let py = self.player.y;
        match event.event_type {
            HistoryEventType::Walk => (),
            HistoryEventType::Push => {
                let (bx, by) = event.direction.reverse().from(px, py);
                if let Some(Entity::Block(block)) = self.entity_at_mut(bx, by) {
                    block.x = px;
                    block.y = py;
                }
            },
            HistoryEventType::Sink(color) => {
                let (bx, by) = event.direction.reverse().from(px, py);
                self.entities.push(Entity::Block(Block::new(px, py, color)));
                self.entities.push(Entity::Water(Water::new(bx, by)));
            },
            HistoryEventType::ColorChange(color) => {
                // TODO: too gentle! hard switch!
                self.set_light_color(color);
            },
        }
        self.player.undo(&event.direction);
    }
}
