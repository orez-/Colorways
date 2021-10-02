use std::collections::HashSet;
use crate::view::GameView;
use piston_window::{clear, Button, RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;

pub struct App {
    view: GameView,
    held_keys: HeldKeys,
}

impl App {
    pub fn new() -> Self {
        App {
            view: GameView::new(),
            held_keys: HeldKeys::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let v = args.viewport();
        gl.draw(v, |_, gl| {
            clear([0xFF as f32, 0xFF as f32, 0xFF as f32, 1.0], gl);
            self.view.render(gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.view.update(args, &self.held_keys);
    }

    pub fn key_press(&mut self, button: &Button) {
        self.held_keys.hold(button);
    }

    pub fn key_release(&mut self, button: &Button) {
        self.held_keys.release(button);
    }
}

pub struct HeldKeys {
    key_set: HashSet<Button>,
    ordered_keys: Vec<Button>,
}

impl HeldKeys {
    fn new() -> Self {
        HeldKeys {
            key_set: HashSet::new(),
            ordered_keys: Vec::new(),
        }
    }

    fn hold(&mut self, button: &Button) {
        if self.key_set.insert(*button) {
            self.ordered_keys.insert(0, *button);
        }
    }

    fn release(&mut self, button: &Button) {
        if self.key_set.remove(button) {
            let index = self.ordered_keys.iter().position(|x| x == button).unwrap();
            self.ordered_keys.remove(index);
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Button> {
        self.ordered_keys.iter()
    }
}
