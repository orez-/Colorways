type ComponentColor = [f32; 4];

const GRAY: ComponentColor = [0.3, 0.3, 0.3, 1.];
const RED: ComponentColor = [1., 0., 0., 1.];
const GREEN: ComponentColor = [0., 1., 0., 1.];
const BLUE: ComponentColor = [0., 0., 1., 1.];
const WHITE: ComponentColor = [1., 1., 1., 1.];

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Gray,
    Red,
    Green,
    Blue,
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
            White => WHITE,
        }
    }

    pub fn as_light_component(&self) -> ComponentColor {
        match self {
            Gray => GRAY,
            Red => [1., 0.3, 0.3, 1.],
            Green => [0.3, 1., 0.3, 1.],
            Blue => [0.3, 0.3, 1., 1.],
            White => WHITE,
        }
    }
}
