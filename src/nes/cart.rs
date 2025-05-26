use super::memory::Memory;

pub struct Cart {
    pub memory: Vec<u8>,
}

impl Memory for Cart {
    fn read(&mut self, address: usize) -> u8 {
        return self.memory[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }
}

impl Cart {
    pub const BEGIN: usize = 0x4020;
    pub const END: usize = 0xFFFF;

    pub fn new(raw_data: Vec<u8>) -> Self {
        return Self {
            memory: raw_data,
        };
    }

    fn map_address(&self, address: usize) -> usize {
        return address - Self::BEGIN;
    }
}
