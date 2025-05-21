use crate::memory::Memory;

pub struct Cart {
    pub memory: Vec<u8>,
    pub mapped_start: usize,
    pub mapped_end: usize,
}

impl Default for Cart {
    fn default() -> Self {
        Self {
            memory: vec![0; Self::DEFAULT_SIZE],
            mapped_start: Self::DEFAULT_BEGIN,
            mapped_end: Self::DEFAULT_END,
        }
    }
}

impl Memory for Cart {
    fn read(&mut self, address: usize) -> u8 {
        let address = self.map_address(address);
        return self.memory[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        let address = self.map_address(address);
        self.memory[address] = value;
    }
}

impl Cart {
    pub const DEFAULT_BEGIN: usize = 0x8000;
    pub const DEFAULT_END: usize = 0xFFFF;
    pub const DEFAULT_SIZE: usize = Self::DEFAULT_END - Self::DEFAULT_BEGIN;

    pub fn new(mapped_start: usize, raw_data: Vec<u8>) -> Self {
        let mapped_end = mapped_start + raw_data.len();
        return Cart {
            memory: raw_data,
            mapped_start: mapped_start,
            mapped_end: mapped_end,
        };
    }

    fn map_address(&self, address: usize) -> usize {
        if (self.mapped_start <= address && address < self.mapped_end) {
            return address - self.mapped_start;
        }
        panic!("invalid memory access");
    }
}
