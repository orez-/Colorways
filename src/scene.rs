use crate::app::Direction;
use crate::color::Color;
use crate::entity::{Block, Entity, IEntity, Player, Water};
use crate::room::Room;
use crate::scene_config::SceneConfig;
use opengl_framebuffer::FrameBuffer;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{Context, DrawState, Image, RenderArgs, Transformed, UpdateArgs};
use piston_window::draw_state::Blend;

const DISPLAY_WIDTH: f64 = 200.;
const DISPLAY_HEIGHT: f64 = 200.;
const DISPLAY_WIDTH_HALF: i64 = DISPLAY_WIDTH as i64 / 2;
const DISPLAY_HEIGHT_HALF: i64 = DISPLAY_HEIGHT as i64 / 2;

pub struct Camera {
    pub x_mode: CameraMode,
    pub y_mode: CameraMode,
}

impl Camera {
    pub fn offset(x: i64, y: i64) -> Self {
        Self {
            x_mode: CameraMode::Fixed(x + DISPLAY_WIDTH_HALF),
            y_mode: CameraMode::Fixed(y + DISPLAY_HEIGHT_HALF),
        }
    }
}

pub enum CameraMode {
    Player,
    Fixed(i64),
    Centered,
}

#[derive(Debug)]
pub enum GameAction {
    Stop,
    Walk,
    Push(usize),
    ColorRadio(Color),
    ColorToggle(Color),
    Win,
    Sink(usize, usize, Color),
}

#[derive(Debug)]
pub enum HistoryEventType {
    Walk,
    Push,
    Sink(Color),
    ColorRadio(Color),
    ColorToggle(Color),
    Win,
}

#[derive(Debug)]
pub struct HistoryEvent {
    pub direction: Direction,
    pub event_type: HistoryEventType,
}

pub enum SceneTag {
    TeachMove,
    TeachUndo,
}

pub struct HeadlessScene {
    player: Player,
    room: Room,
    entities: Vec<Entity>,
    light_color: Color,
}

impl HeadlessScene {
    pub fn new(player: Player, room: Room, entities: Vec<Entity>, starting_color: Color) -> Self {
        let mut this = HeadlessScene {
            player,
            room,
            entities,
            light_color: Color::GRAY,
        };
        this.radio_set_light_color(starting_color);
        this
    }

    /// Flush queued commands.
    /// Actions against the HeadlessScene and its actors may enqueue some
    /// animation or otherwise act with some delay. Since this is undesirable
    /// for puzzle behavior testing, this function will flush any such state
    /// and leave the scene ready to receive the next input.
    #[cfg(test)]
    pub fn resolve(&mut self) {
        self.player.resolve();
    }

    pub fn radio_set_light_color(&mut self, new_color: Color) {
        if self.light_color == new_color { return; }
        let old_color = self.light_color;
        self.light_color = new_color;

        for entity in self.entities.iter_mut() {
            match entity {
                Entity::Lightbulb(bulb) => {
                    if bulb.color == old_color { bulb.turn_off(); }
                    else if bulb.color == new_color { bulb.turn_on(); }
                }
                Entity::Block(block) => {
                    let in_light = self.room.tile_in_light(block.x, block.y, block.color, new_color);
                    block.set_in_light(in_light);
                }
                _ => (),
            }
        }
    }

    pub fn toggle_light_color(&mut self, color: Color) {
        self.light_color ^= color;
        for entity in self.entities.iter_mut() {
            match entity {
                Entity::Lightbulb(bulb) if color == bulb.color => { bulb.toggle(); }
                Entity::Block(block) => {
                    let in_light = self.room.tile_in_light(block.x, block.y, block.color, self.light_color);
                    block.set_in_light(in_light);
                }
                _ => (),
            }
        }
    }

    pub fn entity_id_at(&self, x: i32, y: i32) -> Option<usize> {
        self.entities.iter().position(|e| e.x() == x && e.y() == y)
    }

    pub fn entity_at(&self, x: i32, y: i32) -> Option<&Entity> {
        let idx = self.entity_id_at(x, y)?;
        Some(&self.entities[idx])
    }

    pub fn tile_is_passable(&self, x: i32, y: i32) -> bool {
        let tile = self.room.tile_at(x, y);
        tile.map_or(false, |tile| tile.is_passable())
    }

    pub fn tile_in_light(&self, x: i32, y: i32, tile_color: Color) -> bool {
        self.room.tile_in_light(x, y, tile_color, self.light_color)
    }

    pub fn undo(&mut self, event: HistoryEvent) {
        let px = self.player.x;
        let py = self.player.y;
        match event.event_type {
            HistoryEventType::Walk => (),
            HistoryEventType::Push => {
                let (bx, by) = event.direction.reverse().from(px, py);
                // Ugly. Awful. Rust has failed me on this day.
                let idx = self.entity_id_at(bx, by).unwrap();
                let in_light = if let Entity::Block(block) = &self.entities[idx] {
                    self.room.tile_in_light(px, py, block.color, self.light_color)
                } else { false };
                if let Entity::Block(block) = &mut self.entities[idx] {
                    block.x = px;
                    block.y = py;
                    block.set_in_light(in_light);
                }
            },
            HistoryEventType::Sink(color) => {
                let (bx, by) = event.direction.reverse().from(px, py);
                self.entities.push(Entity::Block(Block::new(px, py, color)));
                self.entities.push(Entity::Water(Water::new(bx, by)));
            },
            HistoryEventType::ColorRadio(color) => {
                // TODO: too gentle! hard switch!
                self.radio_set_light_color(color);
            },
            HistoryEventType::ColorToggle(color) => {
                // TODO: too gentle! hard switch!
                self.toggle_light_color(color);
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
            }
            GameAction::ColorRadio(color) => {
                let evt = HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::ColorRadio(self.light_color),
                };
                self.radio_set_light_color(color);
                evt
            }
            GameAction::ColorToggle(color) => {
                self.toggle_light_color(color);
                HistoryEvent {
                    direction: direction.reverse(),
                    event_type: HistoryEventType::ColorToggle(color),
                }
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
                    let in_light = self.room.tile_in_light(block.x, block.y, block.color, self.light_color);
                    block.set_in_light(in_light);
                } else { unreachable!(); }
                evt
            }
            GameAction::Stop => unreachable!(),
        };
        Some(evt)
    }
}

fn clamp_or_center(c: i64, min: i64, max: i64) -> i64 {
    if min <= max { c.clamp(min, max) }
    else { (min - max) / 2 + max }
}

pub struct Scene {
    texture: GlTexture,
    light_buffer: FrameBuffer,
    pub tag: Option<SceneTag>,
    state: HeadlessScene,
    camera: Camera,
    ambient_color: [f32; 4],
}

impl Scene {
    pub fn new(config: SceneConfig) -> Self {
        let SceneConfig {state, ambient_color, tag, camera } = config;

        let mut texture_settings = opengl_graphics::TextureSettings::new();
        texture_settings.set_mag(opengl_graphics::Filter::Nearest);

        let canvas = image::ImageBuffer::new(800, 800);
        let texture = GlTexture::from_image(&canvas, &texture_settings);
        let light_buffer = FrameBuffer::new(texture);

        Self {
            texture: crate::app::load_texture(),
            light_buffer,
            tag,
            state,
            camera,
            ambient_color,
        }
    }

    // TODO: if the level's too small probably center it instead
    fn camera(&self) -> (i64, i64) {
        let room_width = self.state.room.pixel_width();
        let room_height = self.state.room.pixel_height();
        let (px, py) = self.state.player.center();
        let x = match self.camera.x_mode {
            CameraMode::Player => px,
            CameraMode::Fixed(x) => x,
            CameraMode::Centered => room_width / 2,
        };
        let y = match self.camera.y_mode {
            CameraMode::Player => py,
            CameraMode::Fixed(y) => y,
            CameraMode::Centered => room_height / 2,
        };
        (
            clamp_or_center(x, DISPLAY_WIDTH_HALF, room_width - DISPLAY_WIDTH_HALF),
            clamp_or_center(y, DISPLAY_HEIGHT_HALF, room_height - DISPLAY_HEIGHT_HALF),
        )
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

    pub fn prerender_lights(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let context = self.camera_context2();
        let draw_state = DrawState::default();

        self.light_buffer.draw(args.viewport(), gl, |_, gl| {
            piston_window::clear(self.ambient_color, gl);
            let lights: Vec<_> = self.state.entities.iter().filter_map(|e| {
                if let Entity::Lightbulb(bulb) = e { Some(bulb) }
                else { None }
            }).collect();

            // Pre-paint a darker color over the areas we'll be painting lights.
            // This allows us to keep the lights primarily the color they're displaying,
            // while letting us independently set an ambient light for the scene.
            for light in &lights {
                light.draw_light_base(self.ambient_color, &draw_state, &context, gl);
            }

            for light in lights {
                light.draw_light(&draw_state, &context, gl);
            }
        });
    }

    pub fn render_stuff(&self, draw_state: &DrawState, gl: &mut GlGraphics) {
        let context = self.camera_context();

        self.state.room.render(
            &self.texture,
            draw_state,
            &context,
            gl,
        );

        for entity in &self.state.entities {
            entity.sprite().draw(
                &self.texture,
                draw_state,
                context.transform,
                gl,
            );
        }

        self.state.player.sprite().draw(
            &self.texture,
            draw_state,
            context.transform,
            gl,
        );
    }

    pub fn render_lights(&self, draw_state: &DrawState, gl: &mut GlGraphics) {
        Image::new().draw(
            self.light_buffer.texture(),
            &draw_state.blend(Blend::Multiply),
            Context::new_abs(800., 800.).flip_v().trans(0., -800.).transform,
            gl,
        );
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.state.player.update(args);
        for entity in self.state.entities.iter_mut() {
            entity.update(args);
        }
    }

    // proxied methods
    pub fn player(&self) -> &Player {
        &self.state.player
    }

    pub fn undo(&mut self, event: HistoryEvent) {
        self.state.undo(event);
    }

    pub fn navigate(&mut self, direction: Direction) -> Option<HistoryEvent> {
        self.state.navigate(direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn walk_east(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::East);
        state.resolve();
        assert!(matches!(evt, Some(HistoryEvent {
            event_type: HistoryEventType::Walk,
            direction: Direction::West,
        })), "{evt:?}");
    }

    fn walk_north(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::North);
        state.resolve();
        assert!(matches!(evt, Some(HistoryEvent {
            event_type: HistoryEventType::Walk,
            direction: Direction::South,
        })), "{evt:?}");
    }

    fn walk_south(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::South);
        state.resolve();
        assert!(matches!(evt, Some(HistoryEvent {
            event_type: HistoryEventType::Walk,
            direction: Direction::North,
        })), "{evt:?}");
    }

    fn stopped_east(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::East);
        state.resolve();
        assert!(matches!(evt, None), "{evt:?}");
    }

    fn stopped_north(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::North);
        state.resolve();
        assert!(matches!(evt, None), "{evt:?}");
    }

    fn stopped_south(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::South);
        state.resolve();
        assert!(matches!(evt, None), "{evt:?}");
    }

    fn push_west(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::West);
        state.resolve();
        assert!(matches!(evt, Some(HistoryEvent {
            event_type: HistoryEventType::Push,
            direction: Direction::East,
        })), "{evt:?}");
    }

    fn push_east(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::East);
        state.resolve();
        assert!(matches!(evt, Some(HistoryEvent {
            event_type: HistoryEventType::Push,
            direction: Direction::West,
        })), "{evt:?}");
    }

    fn push_south(state: &mut HeadlessScene) {
        let evt = state.navigate(Direction::South);
        state.resolve();
        assert!(matches!(evt, Some(HistoryEvent {
            event_type: HistoryEventType::Push,
            direction: Direction::North,
        })), "{evt:?}");
    }

    fn push_south_and_advance(state: &mut HeadlessScene) {
        push_south(state);
        walk_north(state);
        walk_east(state);
    }

    fn walk_south_and_advance(state: &mut HeadlessScene) {
        walk_south(state);
        walk_north(state);
        walk_east(state);
    }

    #[test]
    fn test_push() {
        let lvl = include_bytes!("../bin/levels/tests/test_push.skb");
        let SceneConfig { mut state, .. } = SceneConfig::from_file(lvl);

        stopped_north(&mut state);
        push_east(&mut state);
        push_east(&mut state);
        stopped_east(&mut state);
        walk_south(&mut state);
        stopped_south(&mut state);
        push_west(&mut state);
    }

    #[test]
    fn test_primary_push() {
        let lvl = include_bytes!("../bin/levels/tests/test_primary_push.skb");
        let SceneConfig { mut state, .. } = SceneConfig::from_file(lvl);

        // moving thru green light
        // k̽r̽g̽b̽c̽y̽m̽w̽
        push_south_and_advance(&mut state);  // push black
        push_south_and_advance(&mut state);  // push red
        walk_south_and_advance(&mut state);  // walk green
        push_south_and_advance(&mut state);  // push blue
        walk_south_and_advance(&mut state);  // walk cyan
        walk_south_and_advance(&mut state);  // walk yellow
        push_south_and_advance(&mut state);  // push magenta
        walk_south_and_advance(&mut state);  // walk white
    }

    #[test]
    fn test_secondary_push() {
        let lvl = include_bytes!("../bin/levels/tests/test_secondary_push.skb");
        let SceneConfig { mut state, .. } = SceneConfig::from_file(lvl);

        state.navigate(Direction::South);
        state.resolve();

        walk_south(&mut state);

        // moving thru magenta light
        // k̽r̽g̽b̽c̽y̽m̽w̽
        push_south_and_advance(&mut state);  // push black
        push_south_and_advance(&mut state);  // push red
        push_south_and_advance(&mut state);  // push green
        push_south_and_advance(&mut state);  // push blue
        push_south_and_advance(&mut state);  // push cyan
        push_south_and_advance(&mut state);  // push yellow
        walk_south_and_advance(&mut state);  // walk magenta
        walk_south_and_advance(&mut state);  // walk white
    }

    #[test]
    fn test_secondary_obscured() {
        let lvl = include_bytes!("../bin/levels/tests/test_secondary_obscured.skb");
        let SceneConfig { mut state, .. } = SceneConfig::from_file(lvl);

        state.navigate(Direction::South);
        state.resolve();

        walk_south(&mut state);

        // moving thru _red_ light, despite blue also being on
        // k̽r̽g̽b̽c̽y̽m̽w̽
        push_south_and_advance(&mut state);  // push black
        walk_south_and_advance(&mut state);  // walk red
        push_south_and_advance(&mut state);  // push green
        push_south_and_advance(&mut state);  // push blue
        push_south_and_advance(&mut state);  // push cyan
        walk_south_and_advance(&mut state);  // walk yellow
        walk_south_and_advance(&mut state);  // walk magenta
        walk_south_and_advance(&mut state);  // walk white
    }
}
