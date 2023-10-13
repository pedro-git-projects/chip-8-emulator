use std::cmp::Ordering;

use rand::Rng;
use sdl2::event::Event;

use crate::app::app::App;

use super::keyboard::{map_chip8_key_to_sdl_key, map_sdl_key_to_chip8_key, Chip8Key, Keyboard};
use super::screen::Screen;

const MEMORY_CAPACITY: usize = 4096;
const TOTAL_DATA_REGISTERS: usize = 16;

const TOTAL_STACK_DEPTH: u8 = 16;
const CHIP8_DEFAULT_SPRITE_HEIGHT: u16 = 5;
const CHIP8_DEFAULT_CHARACTER_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
const LOAD_ADDRESS: u16 = 0x200;

// unsigend short = u16
// unsigend char = u8

pub struct Chip8 {
    memory: [u8; MEMORY_CAPACITY],
    v: [u8; TOTAL_DATA_REGISTERS], // Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    i: u16, // There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
    stack_pointer: u8,
    stack: [u16; TOTAL_STACK_DEPTH as usize],
    pub keyboard: Keyboard,
    pub screen: Screen,
    v_to_key_map: [Chip8Key; TOTAL_DATA_REGISTERS],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory = [0; MEMORY_CAPACITY];
        let v = [0; TOTAL_DATA_REGISTERS];
        let i = 0;
        let delay_timer = 0;
        let sound_timer = 0;
        let program_counter = 0;
        let stack_pointer = 0;
        let stack = [0; TOTAL_STACK_DEPTH as usize];
        let keyboard = Keyboard::new();
        let screen = Screen::new();
        let v_to_key_map: [Chip8Key; TOTAL_DATA_REGISTERS] = [
            Chip8Key::Key0,
            Chip8Key::Key1,
            Chip8Key::Key2,
            Chip8Key::Key3,
            Chip8Key::Key4,
            Chip8Key::Key5,
            Chip8Key::Key6,
            Chip8Key::Key7,
            Chip8Key::Key8,
            Chip8Key::Key9,
            Chip8Key::KeyA,
            Chip8Key::KeyB,
            Chip8Key::KeyC,
            Chip8Key::KeyD,
            Chip8Key::KeyE,
            Chip8Key::KeyF,
        ];

        let mut chip8 = Chip8 {
            memory,
            v,
            i,
            delay_timer,
            sound_timer,
            program_counter,
            stack_pointer,
            stack,
            keyboard,
            screen,
            v_to_key_map,
        };

        for (i, &value) in CHIP8_DEFAULT_CHARACTER_SET.iter().enumerate() {
            chip8.memory[i] = value;
        }
        chip8
    }

    pub fn set_memory_addr(&mut self, index: usize, value: u8) -> Result<(), &str> {
        match index.cmp(&MEMORY_CAPACITY) {
            Ordering::Less => {
                self.memory[index] = value;
                Ok(())
            }
            _ => Err("memory out of bounds!"),
        }
    }

    pub fn get_memory_addr(&self, index: usize) -> Result<u8, &str> {
        match index.cmp(&MEMORY_CAPACITY) {
            Ordering::Less => Ok(self.memory[index]),
            _ => Err("memory out of bounds!"),
        }
    }

    pub fn push_to_stack(&mut self, index: usize, value: u16) -> Result<(), &str> {
        match self.stack_pointer.cmp(&TOTAL_STACK_DEPTH) {
            Ordering::Less => {
                self.stack[self.stack_pointer as usize] = value;
                self.stack_pointer += 1;
                Ok(())
            }
            _ => Err("memory out of bounds!"),
        }
    }

    pub fn stack_push(&mut self, value: u16) -> Result<(), &'static str> {
        if self.stack_pointer < TOTAL_STACK_DEPTH as u8 {
            self.stack[self.stack_pointer as usize] = value;
            self.stack_pointer += 1;
            Ok(())
        } else {
            Err("memory out of bounds!")
        }
    }

    pub fn pop_from_stack(&mut self) -> Result<u16, &str> {
        match self.stack_pointer.cmp(&0) {
            Ordering::Greater => {
                self.stack_pointer -= 1;
                let val: u16 = self.stack[self.stack_pointer as usize];
                Ok(val)
            }
            _ => Err("memory out of bounds!"),
        }
    }

    pub fn exec(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => self.screen.clear(),
            0x00EE => {
                self.program_counter = self.pop_from_stack().unwrap() as u16;
            }
            _ => self.chip8_exec_extended(opcode),
        }
    }

    pub fn load(&mut self, buf: &[u8]) -> Result<(), &str> {
        let load_address = LOAD_ADDRESS as usize;

        if load_address + buf.len() > MEMORY_CAPACITY {
            return Err("program too large for memory");
        }

        self.memory[load_address..load_address + buf.len()].copy_from_slice(buf);
        self.program_counter = LOAD_ADDRESS;

        Ok(())
    }

    pub fn memory_get_short(&self, index: usize) -> u16 {
        let byte1 = self.memory[index] as u16;
        let byte2 = self.memory[index + 1] as u16;
        (byte1 << 8) | byte2
    }

    fn exec_extended_eight(&mut self, opcode: u16) {
        let x = ((opcode >> 8) & 0x000f) as usize;
        let y = ((opcode >> 4) & 0x000f) as usize;
        let final_four_bits = (opcode & 0x000f) as u8;
        let mut tmp: u8 = 0;

        match final_four_bits {
            // 8xy0 - LD Vx, Vy. Vx = Vy
            0x00 => {
                self.v[x] = self.v[y];
            }
            // 8xy1 - OR Vx, Vy. Performs a bitwise OR on Vx and Vy stores the result in Vx
            0x01 => {
                self.v[x] = self.v[x] | self.v[y];
            }
            // 8xy2 - AND Vx, Vy. Performs a bitwise AND on Vx and Vy stores the result in Vx
            0x02 => {
                self.v[x] = self.v[x] & self.v[y];
            }
            // 8xy3 - XOR Vx, Vy. Performs a bitwise XOR on Vx and Vy stores the result in Vx
            0x03 => {
                self.v[x] = self.v[x] ^ self.v[y];
            }
            // 8xy4 - ADD Vx, Vy. Set Vx = Vx + Vy, set VF = carry
            0x04 => {
                tmp = self.v[x].wrapping_add(self.v[y]);
                self.v[0x0F] = if tmp > 0xFF { 1 } else { 0 };
                self.v[x] = tmp;
            }
            // 8xy5 - SUB Vx, Vy. Set vx = Vx - Vy, set VF = Not borrow
            0x05 => {
                self.v[0x0F] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }
            // 8xy6 - SHR Vx {, Vy}
            0x06 => {
                self.v[0x0F] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            }
            // 8xy7 - SUBN Vx, Vy
            0x07 => {
                self.v[0x0F] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }
            // 8xyE - SHL Vx {, Vy}
            0x0E => {
                self.v[0x0F] = if self.v[x] & 0x80 != 0 { 1 } else { 0 };
                self.v[x] <<= 1;
            }
            _ => {
                // Handle unsupported opcodes or other cases here
            }
        }
    }

    fn chip8_exec_extended_f(&mut self, opcode: u16) {
        let x = ((opcode >> 8) & 0x000F) as usize;
        match opcode & 0x00FF {
            // fx07 - LD Vx, DT. Set Vx to the delay timer value
            0x07 => {
                self.v[x] = self.delay_timer;
            }
            // fx0a - LD Vx, K
            0x0A => {
                if let Some(pressed_key) = self.keyboard.last_key {
                    self.v[x] = pressed_key as u8;
                }
            }
            // fx15 - LD DT, Vx, set the delay timer to Vx
            0x15 => {
                self.delay_timer = self.v[x];
            }
            // fx18 - LD ST, Vx, set the sound timer to Vx
            0x18 => {
                self.sound_timer = self.v[x];
            }
            // fx1e - Add I, Vx
            0x1E => {
                self.i += self.v[x] as u16;
            }
            // fx29 - LD F, Vx
            0x29 => {
                self.i = (self.v[x] as u16) * CHIP8_DEFAULT_SPRITE_HEIGHT;
            }
            // fx33 - LD B, Vx
            0x33 => {
                let hundreds = self.v[x] / 100;
                let tens = (self.v[x] / 10) % 10;
                let units = self.v[x] % 10;
                self.memory[self.i as usize] = hundreds;
                self.memory[(self.i + 1) as usize] = tens;
                self.memory[(self.i + 2) as usize] = units;
            }
            // fx55 - LD [I], Vx
            0x55 => {
                for i in 0..=x {
                    self.memory[(self.i + i as u16) as usize] = self.v[i];
                }
            }
            // fx65 - LD Vx, [I]
            0x65 => {
                for i in 0..=x {
                    self.v[i] = self.memory[(self.i + i as u16) as usize];
                }
            }
            _ => {
                // Handle unsupported opcodes or other cases here
            }
        }
    }

    pub fn chip8_exec_extended(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;
        let x = ((opcode >> 8) & 0x000f) as usize;
        let y = ((opcode >> 4) & 0x000f) as usize;
        let kk = (opcode & 0x00ff) as u8;
        let n = (opcode & 0x000f) as u8;

        match opcode & 0xf000 {
            // 1nnn - JP addr, Jump to location nnn
            0x1000 => self.program_counter = nnn,

            // 2nnn - CALL addr, Call subroutine at location nnn
            0x2000 => {
                self.stack_push(self.program_counter).unwrap();
                self.program_counter = nnn;
            }

            // 3xkk - SE Vx, byte, Skip next instruction if Vx == kk
            0x3000 => {
                if self.v[x] == kk {
                    self.program_counter += 2;
                }
            }

            // 4xkk - SNE Vx, byte, Skip next instruction if Vx != kk
            0x4000 => {
                if self.v[x] != kk {
                    self.program_counter += 2;
                }
            }

            // 5xy0 - SE Vx, Vy, Skip next instruction if Vx == Vy
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.program_counter += 2;
                }
            }

            // 6xkk - LD Vx, byte, Set Vx = kk
            0x6000 => self.v[x] = kk,

            // 7xkk - ADD Vx, byte, Set Vx = Vx + kk
            0x7000 => {
                let result = self.v[x].wrapping_add(kk);
                self.v[x] = result;
            }

            // 8xyN - Various operations, delegate to another function
            0x8000 => self.exec_extended_eight(opcode),

            // 9xy0 - SNE Vx, Vy, Skip next instruction if Vx != Vy
            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.program_counter += 2;
                }
            }

            // Annn - LD I, addr, Set I register to nnn
            0xA000 => self.i = nnn,

            // Bnnn - JP V0, addr, Jump to location nnn + V0
            0xB000 => self.program_counter = nnn + self.v[0x00] as u16,

            // Cxkk - RND Vx, byte
            0xC000 => {
                let mut rng = rand::thread_rng();
                let random_byte = rng.gen::<u8>();
                self.v[x] = random_byte & kk;
            }

            // Dxyn - DRW Vx, Vy, nibble. Draws sprite to the screen
            0xD000 => {
                let sprite = &self.memory[(self.i as usize)..(self.i as usize + n as usize)];
                let collision = self.screen.chip8_screen_draw_sprite(
                    self.v[x].into(),
                    self.v[y].into(),
                    sprite,
                    n.into(),
                );
                self.v[0x0F] = if collision { 1 } else { 0 };
            }

            // ExNN - Keyboard operations
            0xE000 => {
                match opcode & 0x00ff {
                    // Ex9E - SKP Vx, Skip the next instruction if the key with the value of Vx is pressed
                    0x9E => {
                        let vx = self.v_to_key_map[x];
                        let kc = map_chip8_key_to_sdl_key(vx);
                        if self.keyboard.is_key_down(kc) {
                            self.program_counter += 2;
                        }
                    }
                    // ExA1 - SKNP Vx, Skip the next instruction if the key with the value of Vx is not pressed
                    0xA1 => {
                        let vx = self.v_to_key_map[x];
                        let kc = map_chip8_key_to_sdl_key(vx);
                        if !self.keyboard.is_key_down(kc) {
                            self.program_counter += 2;
                        }
                    }
                    _ => {} // Handle unsupported opcodes or other cases here
                }
            }

            // FxNN - Miscellaneous operations, delegate to another function
            0xF000 => self.chip8_exec_extended_f(opcode),

            _ => {} // Handle unsupported opcodes or other cases here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_memory_addr_within_bounds() {
        let mut chip8 = Chip8::new();
        let index = 0;
        let value = 0x90;
        let result = chip8.set_memory_addr(index, value);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_set_memory_addr_out_of_bounds() {
        let mut chip8 = Chip8::new();
        let index = MEMORY_CAPACITY + 1;
        let value = 0x90;
        let result = chip8.set_memory_addr(index, value);
        assert_eq!(result, Err("memory out of bounds!"));
    }

    #[test]
    fn test_get_memory_addr_within_bounds() {
        let mut chip8 = Chip8::new();
        let index = 0;
        chip8.set_memory_addr(index, 0x90).unwrap();
        let result = chip8.get_memory_addr(index);
        assert_eq!(result, Ok(0x90));
    }

    #[test]
    fn test_get_memory_addr_out_of_bounds() {
        let chip8 = Chip8::new();
        let index = MEMORY_CAPACITY + 1;
        let result = chip8.get_memory_addr(index);
        assert_eq!(result, Err("memory out of bounds!"));
    }

    #[test]
    fn test_push_to_stack_within_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = 0;
        let value = 0x200;
        let result = chip8.push_to_stack(0, value);
        assert_eq!(result, Ok(()));
        assert_eq!(chip8.stack[0], 0x200);
        assert_eq!(chip8.stack_pointer, 1);
    }

    #[test]
    fn test_push_to_stack_out_of_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = TOTAL_STACK_DEPTH; // Set the stack pointer to the maximum value
        let value = 0x200;
        let result = chip8.push_to_stack(TOTAL_STACK_DEPTH as usize, value);
        assert_eq!(result, Err("memory out of bounds!"));
    }

    #[test]
    fn test_pop_from_stack_within_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = 1;
        chip8.stack[0] = 0x200;
        let result = chip8.pop_from_stack();
        assert_eq!(result, Ok(0x200));
        assert_eq!(chip8.stack_pointer, 0);
    }

    #[test]
    fn test_pop_from_stack_out_of_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = 0; // Set the stack pointer to 0
        let result = chip8.pop_from_stack();
        assert_eq!(result, Err("memory out of bounds!"));
    }

    #[test]
    fn test_load_valid_program() {
        let mut chip8 = Chip8::new();
        let program: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let result = chip8.load(&program);
        assert_eq!(result, Ok(()));
        assert_eq!(chip8.program_counter, LOAD_ADDRESS);
    }

    #[test]
    fn test_load_program_too_large() {
        let mut chip8 = Chip8::new();
        let program: [u8; MEMORY_CAPACITY + 1] = [0; MEMORY_CAPACITY + 1];
        let result = chip8.load(&program);
        assert_eq!(result, Err("program too large for memory"));
    }
}
