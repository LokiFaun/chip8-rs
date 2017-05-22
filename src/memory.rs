pub const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: [0; MEMORY_SIZE] }
    }

    pub fn load8(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn load16(&self, index: usize) -> u16 {
        ((self.memory[index] as u16) << 8) + self.memory[index + 1] as u16
    }

    pub fn store(&mut self, start: usize, array: &[u8]) {
        for (index, element) in array.into_iter().enumerate() {
            self.memory[start + index] = *element;
        }
    }

    pub fn store8(&mut self, index: usize, element: u8) {
        self.memory[index] = element
    }
}

