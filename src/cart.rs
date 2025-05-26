use crate::memory::Memory;
pub struct Cart<F: Fn(usize) -> usize> {
    pub memory: Vec<u8>,
    pub mmap: F,
}

impl<F: Fn(usize) -> usize> Memory for Cart<F> {
    fn read(&mut self, address: usize) -> u8 {
        let address = (self.mmap)(address);
        return self.memory[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        let address = (self.mmap)(address);
        self.memory[address] = value;
    }
}

impl<F: Fn(usize) -> usize> Cart<F> {
    pub const BEGIN: usize = 0x4020;
    pub const END: usize = 0xFFFF;

    pub fn new(raw_data: Vec<u8>, mmap: F) -> Self {
        return Self {
            memory: raw_data,
            mmap: mmap,
        };
    }
}
