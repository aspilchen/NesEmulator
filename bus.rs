use crate::{cpu::Cpu, memory::Memory, ram::Ram};

pub struct Bus {
    pub cpu: Cpu,
    pub ram: Ram,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            ram: Default::default(),
        }
    }
}

impl Memory for Bus {
    fn read(&self, address: usize) -> u8 {
        return self.ram.read(address);
    }

    fn write(&mut self, address: usize, value: u8) {
        self.ram.write(address, value);
    }
}
