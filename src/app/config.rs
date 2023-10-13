const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const CHIP8_WINDOW_MULTIPLIER: u32 = 10;

#[derive(Debug, Clone, Copy)]
pub struct WindowConfig {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

impl WindowConfig {
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        WindowConfig {
            title,
            width,
            height,
        }
    }

    pub fn default() -> Self {
        WindowConfig {
            title: "CHIP-8 Emulator",
            width: CHIP8_WIDTH * CHIP8_WINDOW_MULTIPLIER,
            height: CHIP8_HEIGHT * CHIP8_WINDOW_MULTIPLIER,
        }
    }
}
