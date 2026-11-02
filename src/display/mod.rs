use std::io::{stdout, Write};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pixels: [bool; WIDTH * HEIGHT],
    dirty: bool
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT],
            dirty: false
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * WIDTH + x]
    }

    pub fn print_display(&mut self) {
        if !self.dirty {
            return;
        }

        let mut frame = String::with_capacity(WIDTH * HEIGHT * 6);
        frame.push_str("\x1B[H");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                frame.push_str(if self.get_pixel(x, y) { "██" } else { "  " });
            }
            if y + 1 < HEIGHT {
                frame.push_str("\r\n");
            }
        }

        let mut stdout = stdout().lock();
        stdout.write_all(frame.as_bytes()).expect("error while writing frame to stdout");
        stdout.flush().expect("error while flushing stdout");

        self.dirty = false;
    }

    pub fn clear_display(&mut self) {
        self.pixels.fill(false);
        self.dirty = true;
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> bool {
        let mut collision = false;

        for (row, byte) in sprite.iter().enumerate() {
            for col in 0..8 {
                let sprite_pixel = (byte & (0x80 >> col)) != 0;

                if sprite_pixel {
                    let px = (x + col) % WIDTH;
                    let py = (y + row) % HEIGHT;

                    let index = py * WIDTH + px;

                    if self.pixels[index] {
                        collision = true;
                    }

                    self.pixels[index] ^= true;
                }
            }
        }

        self.dirty = true;
        collision
    }
}
