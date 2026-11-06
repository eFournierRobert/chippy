use std::io::{Write, stdout};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

/// A 64-by-32 pixel display with terminal rendering.
pub struct Display {
    pixels: [bool; WIDTH * HEIGHT],
    dirty: bool,
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl Display {
    /// Creates a blank display.
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT],
            dirty: false,
        }
    }

    /// Returns the pixel at `(x, y)`.
    ///
    /// # Panics
    ///
    /// Panics if the calculated pixel index is outside the display buffer.
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * WIDTH + x]
    }

    /// Writes the display to standard output if it has changed.
    ///
    /// # Panics
    ///
    /// Panics if writing or flushing standard output fails.
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
        stdout
            .write_all(frame.as_bytes())
            .expect("error while writing frame to stdout");
        stdout.flush().expect("error while flushing stdout");

        self.dirty = false;
    }

    /// Clears all pixels and marks the display for rendering.
    pub fn clear_display(&mut self) {
        self.pixels.fill(false);
        self.dirty = true;
    }

    /// XORs a sprite onto the display, wrapping at the edges.
    ///
    /// Returns `true` if any lit pixel was erased.
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
