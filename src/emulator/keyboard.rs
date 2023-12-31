use sdl2::keyboard::Keycode;

#[derive(Debug, Clone, Copy)]
pub enum Chip8Key {
    Key1,
    Key2,
    Key3,
    KeyC,
    Key4,
    Key5,
    Key6,
    KeyD,
    Key7,
    Key8,
    Key9,
    KeyE,
    KeyA,
    Key0,
    KeyB,
    KeyF,
}

pub struct Keyboard {
    keys: [bool; 16],
    pub last_key: Option<Chip8Key>,
}

pub fn map_sdl_key_to_chip8_key(sdl_key: Keycode) -> Option<Chip8Key> {
    match sdl_key {
        Keycode::Num1 => Some(Chip8Key::Key1),
        Keycode::Num2 => Some(Chip8Key::Key2),
        Keycode::Num3 => Some(Chip8Key::Key3),
        Keycode::C => Some(Chip8Key::KeyC),
        Keycode::Num4 => Some(Chip8Key::Key4),
        Keycode::Num5 => Some(Chip8Key::Key5),
        Keycode::Num6 => Some(Chip8Key::Key6),
        Keycode::D => Some(Chip8Key::KeyD),
        Keycode::Num7 => Some(Chip8Key::Key7),
        Keycode::Num8 => Some(Chip8Key::Key8),
        Keycode::Num9 => Some(Chip8Key::Key9),
        Keycode::E => Some(Chip8Key::KeyE),
        Keycode::A => Some(Chip8Key::KeyA),
        Keycode::Num0 => Some(Chip8Key::Key0),
        Keycode::B => Some(Chip8Key::KeyB),
        Keycode::F => Some(Chip8Key::KeyF),
        _ => None,
    }
}

pub fn map_chip8_key_to_sdl_key(chip8_key: Chip8Key) -> Keycode {
    match chip8_key {
        Chip8Key::Key1 => Keycode::Num1,
        Chip8Key::Key2 => Keycode::Num2,
        Chip8Key::Key3 => Keycode::Num3,
        Chip8Key::KeyC => Keycode::C,
        Chip8Key::Key4 => Keycode::Num4,
        Chip8Key::Key5 => Keycode::Num5,
        Chip8Key::Key6 => Keycode::Num6,
        Chip8Key::KeyD => Keycode::D,
        Chip8Key::Key7 => Keycode::Num7,
        Chip8Key::Key8 => Keycode::Num8,
        Chip8Key::Key9 => Keycode::Num9,
        Chip8Key::KeyE => Keycode::E,
        Chip8Key::KeyA => Keycode::A,
        Chip8Key::Key0 => Keycode::Num0,
        Chip8Key::KeyB => Keycode::B,
        Chip8Key::KeyF => Keycode::F,
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: [false; 16],
            last_key: None,
        }
    }

    pub fn key_down(&mut self, key: Keycode) {
        if let Some(chip8_key) = map_sdl_key_to_chip8_key(key) {
            self.keys[chip8_key as usize] = true;
            self.last_key = Some(chip8_key);
        }
    }

    pub fn key_up(&mut self, key: Keycode) {
        if let Some(chip8_key) = map_sdl_key_to_chip8_key(key) {
            self.keys[chip8_key as usize] = false;
            self.last_key = None;
        }
    }

    pub fn is_key_down(&self, key: Keycode) -> bool {
        if let Some(chip8_key) = map_sdl_key_to_chip8_key(key) {
            return self.keys[chip8_key as usize];
        }
        false
    }
}
