use std::cmp::Ordering;

use super::keyboard::Keyboard;
use super::screen::Screen;

const MEMORY_CAPACITY: usize = 4096;
const TOTAL_DATA_REGISTERS: usize = 16;
const TOTAL_STACK_DEPTH: u8 = 16;
const CHIP8_DEFAULT_CHARACTER_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

// unsigend short = u16
// unsigend char = u8

pub struct Chip8 {
    memory: [u8; MEMORY_CAPACITY],
    v: [char; TOTAL_DATA_REGISTERS], // Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    i: u16, // There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    pub delay_timer: u8,
    pub sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [char; TOTAL_STACK_DEPTH as usize],
    pub keyboard: Keyboard,
    pub screen: Screen,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory = [0; MEMORY_CAPACITY];
        let v = ['\0'; TOTAL_DATA_REGISTERS];
        let i = 0;
        let delay_timer = 0;
        let sound_timer = 0;
        let program_counter = 0;
        let stack_pointer = 0;
        let stack = ['\0'; TOTAL_STACK_DEPTH as usize];
        let keyboard = Keyboard::new();
        let screen = Screen::new();

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

    pub fn push_to_stack(&mut self, index: usize, value: char) -> Result<(), &str> {
        match self.stack_pointer.cmp(&TOTAL_STACK_DEPTH) {
            Ordering::Less => {
                self.stack[self.stack_pointer as usize] = value;
                self.stack_pointer += 1;
                Ok(())
            }
            _ => Err("memory out of bounds!"),
        }
    }

    pub fn pop_from_stack(&mut self) -> Result<char, &str> {
        match self.stack_pointer.cmp(&0) {
            Ordering::Greater => {
                self.stack_pointer -= 1;
                let val: char = self.stack[self.stack_pointer as usize];
                Ok(val)
            }
            _ => Err("memory out of bounds!"),
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
        let value = 'A';
        let result = chip8.push_to_stack(0, value);
        assert_eq!(result, Ok(()));
        assert_eq!(chip8.stack[0], 'A');
        assert_eq!(chip8.stack_pointer, 1);
    }

    #[test]
    fn test_push_to_stack_out_of_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = TOTAL_STACK_DEPTH; // Set the stack pointer to the maximum value
        let value = 'A';
        let result = chip8.push_to_stack(TOTAL_STACK_DEPTH as usize, value);
        assert_eq!(result, Err("memory out of bounds!"));
    }

    #[test]
    fn test_pop_from_stack_within_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = 1;
        chip8.stack[0] = 'A';
        let result = chip8.pop_from_stack();
        assert_eq!(result, Ok('A'));
        assert_eq!(chip8.stack_pointer, 0);
    }

    #[test]
    fn test_pop_from_stack_out_of_bounds() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = 0; // Set the stack pointer to 0
        let result = chip8.pop_from_stack();
        assert_eq!(result, Err("memory out of bounds!"));
    }
}
