use std::cmp::Ordering;

const MEMORY_CAPACITY: usize = 4096;
const TOTAL_DATA_REGISTERS: usize = 16;

// unsigend short = u16
// unsigend char = u8

pub struct Chip8 {
    memory: [char; MEMORY_CAPACITY],
    v: [char; TOTAL_DATA_REGISTERS], // Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    i: u16, // There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory = ['\0'; MEMORY_CAPACITY]; // Initialize memory with null characters
        let v = ['\0'; TOTAL_DATA_REGISTERS];
        let i = 0;
        let delay_timer = 0;
        let sound_timer = 0;
        let program_counter = 0;
        let stack_pointer = 0;

        Chip8 {
            memory,
            v,
            i,
            delay_timer,
            sound_timer,
            program_counter,
            stack_pointer,
        }
    }
    pub fn set_memory_addr(&mut self, index: usize, value: char) -> Result<(), &str> {
        match index.cmp(&MEMORY_CAPACITY) {
            Ordering::Less => {
                self.memory[index] = value;
                Ok(())
            }
            _ => Err("memory out of bounds!"),
        }
    }

    pub fn get_memory_addr(&self, index: usize) -> Result<char, &str> {
        match index.cmp(&MEMORY_CAPACITY) {
            Ordering::Less => Ok(self.memory[index]),
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
        let value = 'A';
        let result = chip8.set_memory_addr(index, value);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_set_memory_addr_out_of_bounds() {
        let mut chip8 = Chip8::new();
        let index = MEMORY_CAPACITY + 1;
        let value = 'A';
        let result = chip8.set_memory_addr(index, value);
        assert_eq!(result, Err("memory out of bounds!"));
    }

    #[test]
    fn test_get_memory_addr_within_bounds() {
        let mut chip8 = Chip8::new();
        let index = 0;
        chip8.set_memory_addr(index, 'A').unwrap();
        let result = chip8.get_memory_addr(index);
        assert_eq!(result, Ok('A'));
    }

    #[test]
    fn test_get_memory_addr_out_of_bounds() {
        let chip8 = Chip8::new();
        let index = MEMORY_CAPACITY + 1;
        let result = chip8.get_memory_addr(index);
        assert_eq!(result, Err("memory out of bounds!"));
    }
}
