pub trait Memory {
    fn read(&self, address: usize) -> u8;
    // fn read_16(&self, address: usize) -> u16;
    fn write(&mut self, address: usize, value: u8);
}