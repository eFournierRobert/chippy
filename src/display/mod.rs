const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * WIDTH + x]
    }

    pub fn print_display(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", if self.get_pixel(x, y) { "██" } else { "  " });
            }
            println!();
        }
    }

    pub fn clear_display(&mut self) {
        self.pixels.fill(false);
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> bool {
        let mut collision = false;

        for (row, byte) in sprite.iter().enumerate() {
            for col in 0..8 {
                let sprite_pixel = (byte & (0x80 >> col)) != 0;

                if sprite_pixel {
                    let index = (y + row) * WIDTH + (x + col);

                    if self.pixels[index] {
                        collision = true;
                    }

                    self.pixels[index] ^= true;
                }
            }
        }

        collision
    }
}
