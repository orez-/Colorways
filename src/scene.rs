use crate::app::Direction;
use crate::color::Color;
use crate::entity::{Block, Entity, IEntity, Player, Water};
use crate::room::Room;
use opengl_framebuffer::FrameBuffer;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, RenderArgs, Transformed, UpdateArgs};
use piston_window::draw_state::Blend;

const DARK: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const DISPLAY_WIDTH_HALF: i64 = DISPLAY_WIDTH as i64 / 2;
const DISPLAY_HEIGHT_HALF: i64 = DISPLAY_HEIGHT as i64 / 2;

enum CameraMode {
    Player,
}

#[derive(Debug)]
pub enum GameAction {
    Stop,
    Walk,
    Push(usize),
    ColorChange(Color),
    Win,
    Sink(usize, usize, Color),
}

pub enum HistoryEventType {
    Walk,
    Push,
    Sink(Color),
    ColorChange(Color),
    Win,
}

pub struct HistoryEvent {
    pub direction: Direction,
    pub event_type: HistoryEventType,
}

pub struct Scene {
    texture: GlTexture,
    light_buffer: FrameBuffer,
    pub player: Player,
    room: Room,
    entities: Vec<Entity>,
    light_color: Color,
    camera_mode: CameraMode,
}

impl Scene {
    pub fn new(level_id: usize) -> Self {
        let mut texture_settings = opengl_graphics::TextureSettings::new();
        texture_settings.set_mag(opengl_graphics::Filter::Nearest);

        let canvas = image::ImageBuffer::new(800, 800);
        let texture = GlTexture::from_image(&canvas, &texture_settings);
        let light_buffer = FrameBuffer::new(texture);

        let (room, player, entities, light_color) = Room::new(level_id);

        let mut this = Self {
            texture: crate::app::load_texture(),
            light_buffer,
            player,
            room,
            entities,
            light_color: Color::GRAY,
            camera_mode: CameraMode::Player,
        };
        this.set_light_color(light_color);
        this
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

    // TODO: if the level's too small probably center it instead
    fn camera(&self) -> (i64, i64) {
        match self.camera_mode {
            CameraMode::Player => {
                let (x, y) = self.player.center();
                (
                    x.clamp(DISPLAY_WIDTH_HALF, self.room.pixel_width() - DISPLAY_WIDTH_HALF),
                    y.clamp(DISPLAY_HEIGHT_HALF, self.room.pixel_height() - DISPLAY_HEIGHT_HALF),
                )
            }
        }
    }

    fn absolute_context(&self) -> Context {
        Context::new_abs(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    pub fn camera_context(&self) -> Context {
        let (x, y) = self.camera();
        let x = x as f64;
        let y = y as f64;
        self.absolute_context()
            .trans(-x + DISPLAY_WIDTH / 2., -y + DISPLAY_HEIGHT / 2.)
    }

    fn camera_context2(&self) -> Context {
        let (x, y) = self.camera();
        let x = x as f64;
        let y = y as f64;
        Context::new_abs(800., 800.)
            .zoom(4.)
            .trans(-x + DISPLAY_WIDTH / 2., -y + DISPLAY_HEIGHT / 2.)
    }

    pub fn render_lights(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let context = self.camera_context2();
        let draw_state = DrawState::default();

        self.light_buffer.draw(args.viewport(), gl, |_, gl| {
            piston_window::clear(DARK, gl);
            let lights: Vec<_> = self.entities.iter().filter_map(|e| {
                if let Entity::Lightbulb(bulb) = e { Some(bulb) }
                else { None }
            }).collect();

            for light in lights {
                light.draw_light(&context, &draw_state, gl);
            }
        });
    }

    pub fn render_game(&mut self, draw_state: &DrawState, gl: &mut GlGraphics) {
        // Camera
        let context = self.camera_context();

        // Action
        self.room.render(
            &self.texture,
            draw_state,
            &context,
            gl,
        );

        for entity in &self.entities {
            entity.sprite().draw(
                &self.texture,
                draw_state,
                context.transform,
                gl,
            );
        }

        self.player.sprite().draw(
            &self.texture,
            draw_state,
            context.transform,
            gl,
        );

        // Lights
        Image::new().draw(
            self.light_buffer.texture(),
            &draw_state.blend(Blend::Multiply),
            Context::new_abs(800., 800.).flip_v().trans(0., -800.).transform,
            gl,
        );
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.player.update(args);
        for entity in self.entities.iter_mut() {
            entity.update(args);
        }
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

    pub fn tile_is_passable(&self, x: i32, y: i32) -> bool {
        let tile = self.room.tile_at(x, y);
        tile.map_or(false, |tile| tile.is_passable())
    }

    pub fn tile_in_light(&self, x: i32, y: i32, tile_color: Color) -> bool {
        // println!("light: {:?}; tile: {:?}", self.light_color, tile_color);
        if self.light_color == Color::GRAY { return false; }
        if tile_color == Color::WHITE { return true; }
        // println!("contains? {:?}", tile_color.contains(self.light_color));
        tile_color.contains(self.light_color)
            && self.room.tile_in_light(x, y, self.light_color)
    }

    pub fn undo(&mut self, event: HistoryEvent) {
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
            HistoryEventType::Win => (),
        }
        self.player.undo(event.direction);
    }

    pub fn navigate(&mut self, direction: Direction) -> Option<HistoryEvent> {
        self.player.face(direction);
        let (nx, ny) = direction.from(self.player.x, self.player.y);
        if self.player.can_walk() && self.tile_is_passable(nx, ny) {
            let action = if let Some(entity_id) = self.entity_id_at(nx, ny) {
                self.entities[entity_id].on_approach(entity_id, direction, self)
            }
            else { GameAction::Walk };
            return self.handle_action(direction, action);
        }
        None
    }

    fn handle_action(&mut self, direction: Direction, action: GameAction) -> Option<HistoryEvent> {
        if matches!(action, GameAction::Stop) { return None; }
        self.player.walk(direction);
        let evt = match action {
            GameAction::Walk => {
                HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Walk,
                }
            },
            GameAction::ColorChange(color) => {
                let evt = HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::ColorChange(self.light_color.clone()),
                };
                self.set_light_color(color);
                evt
            }
            GameAction::Win => {
                HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Win,
                }
            }
            GameAction::Sink(idx1, idx2, color) => {
                let evt = HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Sink(color),
                };
                let mut idx = 0;
                self.entities.retain(|_| {
                    let m = idx1 != idx && idx2 != idx;
                    idx += 1;
                    m
                });
                evt
            }
            GameAction::Push(entity_id) => {
                let evt = HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::Push,
                };
                if let Entity::Block(block) = &mut self.entities[entity_id] {
                    block.push(direction);
                } else { unreachable!(); }
                evt
            }
            GameAction::Stop => unreachable!(),
        };
        Some(evt)
    }
}
