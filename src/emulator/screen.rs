const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

pub struct Screen {
    pixels: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: [[false; CHIP8_WIDTH]; CHIP8_HEIGHT],
        }
    }

    pub fn check_bounds(&self, x: usize, y: usize) {
        assert!(
            x < CHIP8_WIDTH && y < CHIP8_HEIGHT,
            "Pixel coordinates out of bounds"
        );
    }

    pub fn set_screen(&mut self, x: usize, y: usize) {
        self.check_bounds(x, y);
        self.pixels[y][x] = true;
    }

    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = false;
            }
        }
    }

    pub fn is_set(&self, x: usize, y: usize) -> bool {
        self.check_bounds(x, y);
        self.pixels[y][x]
    }

    pub fn chip8_screen_draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8], num: usize) -> bool {
        let mut pixel_collision = false;

        for (ly, &c) in sprite.iter().enumerate().take(num) {
            for lx in 0..8 {
                if (c & (0b10000000 >> lx)) == 0 {
                    continue;
                }

                let screen_x = (x + lx) % CHIP8_WIDTH;
                let screen_y = (y + ly) % CHIP8_HEIGHT;

                if self.pixels[screen_y][screen_x] {
                    pixel_collision = true;
                }

                self.pixels[screen_y][screen_x] ^= true;
            }
        }

        pixel_collision
    }
}
