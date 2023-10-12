use std::cmp::Ordering;

const MEMORY_CAPACITY: usize = 4096;

pub struct Chip8 {
    memory: [char; MEMORY_CAPACITY],
    width: u8,
    height: u8,
    scale_factor: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory = ['\0'; MEMORY_CAPACITY]; // Initialize memory with null characters
        let width = 64;
        let height = 32;
        let scale_factor = 10;

        Chip8 {
            memory,
            width,
            height,
            scale_factor,
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
