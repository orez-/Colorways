use piston_window::Image;

#[derive(Clone, Copy)]
pub struct Decal {
    pub src_left: f64,
    pub src_top: f64,
    pub dest_left: f64,
    pub dest_top: f64,
    pub width: f64,
    pub height: f64,
}

impl Decal {
    pub const fn src_rect(&self) -> [f64; 4] {
        [self.src_left, self.src_top, self.width, self.height]
    }

    pub const fn dest_rect(&self) -> [f64; 4] {
        [self.dest_left, self.dest_top, self.width, self.height]
    }

    pub fn sprite(&self) -> Image {
        Image::new()
            .src_rect(self.src_rect())
            .rect(self.dest_rect())
    }
}
