use crate::memory::Memory;

pub struct Ram {
    pub memory: [u8; 0xFFFF],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}

impl Memory for Ram {
    fn read(&self, address: usize) -> u8 {
        return self.memory[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }
    
}