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
            width: 640,
            height: 320,
        }
    }
}
