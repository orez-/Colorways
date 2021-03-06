type ComponentColor = [f32; 4];

const GRAY: ComponentColor = [0.3, 0.3, 0.3, 1.];
const RED: ComponentColor = [1., 0., 0., 1.];
const GREEN: ComponentColor = [0., 1., 0., 1.];
const BLUE: ComponentColor = [0., 0., 1., 1.];
const YELLOW: ComponentColor = [1., 1., 0., 1.];
const CYAN: ComponentColor = [0., 1., 1., 1.];
const MAGENTA: ComponentColor = [1., 0., 1., 1.];
const WHITE: ComponentColor = [1., 1., 1., 1.];

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Gray,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
}
use Color::*;

impl Color {
    pub fn as_component(&self) -> ComponentColor {
        match self {
            Gray => GRAY,
            Red => RED,
            Green => GREEN,
            Blue => BLUE,
            Yellow => YELLOW,
            Cyan => CYAN,
            Magenta => MAGENTA,
            White => WHITE,
        }
    }

    pub fn as_light_component(&self) -> ComponentColor {
        match self {
            Gray => GRAY,
            Red => [1., 0.2, 0.2, 1.],
            Green => [0.2, 1., 0.2, 1.],
            Blue => [0.2, 0.2, 1., 1.],
            White => WHITE,
            _ => unimplemented!(),
        }
    }

    pub fn contains(&self, subcolor: &Color) -> bool {
        match (self, subcolor) {
            (Red, Red) => true,
            (Green, Green) => true,
            (Blue, Blue) => true,
            (Yellow, Red | Green) => true,
            (Cyan, Green | Blue) => true,
            (Magenta, Red | Blue) => true,
            (White, Red | Green | Blue) => true,
            _ => false,
        }
    }
}
