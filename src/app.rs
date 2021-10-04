use std::collections::HashSet;
use crate::view::GameView;
use crate::menus::MenuView;
use piston_window::{clear, Button, RenderArgs, UpdateArgs};
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

const AMBIENT_LUM: f32 = 0.6;

const TEXTURE: &[u8] = include_bytes!("../bin/spritesheet.png");

pub fn load_texture() -> GlTexture {
    let mut texture_settings = opengl_graphics::TextureSettings::new();
    texture_settings.set_mag(Filter::Nearest);

    let img = image::load_from_memory(TEXTURE).unwrap();
    let img = match img {
        image::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba8(),
    };
    GlTexture::from_image(&img, &texture_settings)
}

pub struct App {
    view: MenuView,
    held_keys: HeldKeys,
}

impl App {
    pub fn new() -> Self {
        App {
            view: MenuView::new(),
            held_keys: HeldKeys::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let v = args.viewport();
        gl.draw(v, |_, gl| {
            clear([AMBIENT_LUM, AMBIENT_LUM, AMBIENT_LUM, 1.0], gl);
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
