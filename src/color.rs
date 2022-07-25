use bitflags::bitflags;

type ComponentColor = [f32; 4];

const GRAY: ComponentColor = [0.3, 0.3, 0.3, 1.];
const RED: ComponentColor = [1., 0., 0., 1.];
const GREEN: ComponentColor = [0., 1., 0., 1.];
const BLUE: ComponentColor = [0., 0., 1., 1.];
const YELLOW: ComponentColor = [1., 1., 0., 1.];
const CYAN: ComponentColor = [0., 1., 1., 1.];
const MAGENTA: ComponentColor = [1., 0., 1., 1.];
const WHITE: ComponentColor = [1., 1., 1., 1.];

bitflags! {
    pub struct Color: u8 {
        const GRAY = 0;
        const RED = 1;
        const GREEN = 2;
        const BLUE = 4;
        const YELLOW = Self::RED.bits | Self::GREEN.bits;
        const CYAN = Self::GREEN.bits | Self::BLUE.bits;
        const MAGENTA = Self::RED.bits | Self::BLUE.bits;
        const WHITE = Self::RED.bits | Self::GREEN.bits | Self::BLUE.bits;
    }
}

impl Color {
    pub fn as_component(&self) -> ComponentColor {
        // use Color::{Gray, Red, Green, Blue, Yellow, Cyan, Magenta, White};

        match *self {
            Color::GRAY => GRAY,
            Color::RED => RED,
            Color::GREEN => GREEN,
            Color::BLUE => BLUE,
            Color::YELLOW => YELLOW,
            Color::CYAN => CYAN,
            Color::MAGENTA => MAGENTA,
            Color::WHITE => WHITE,
            _ => panic!(),
        }
    }

    pub fn as_light_component(&self) -> ComponentColor {
        match *self {
            Color::GRAY => GRAY,
            Color::RED => [1., 0.2, 0.2, 1.],
            Color::GREEN => [0.2, 1., 0.2, 1.],
            Color::BLUE => [0.2, 0.2, 1., 1.],
            Color::WHITE => WHITE,
            _ => unimplemented!(),
        }
    }
}
