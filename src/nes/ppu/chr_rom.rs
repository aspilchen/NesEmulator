use crate::memory::Memory;

pub struct ChrRom {
    pub memory: Vec<u8>,
}

impl Memory for ChrRom {
    fn read(&mut self, address: usize) -> u8 {
        todo!()
    }

    fn write(&mut self, address: usize, value: u8) {
        todo!()
    }
}