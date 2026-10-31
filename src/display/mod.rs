const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pixels: [bool; WIDTH * HEIGHT]
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT]
        }
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y * WIDTH * x] = !self.pixels[y * WIDTH * x]
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.pixels[y * WIDTH * x]
    }

    pub fn clear_display(&mut self) {
        self.pixels.fill(false);
    }
}