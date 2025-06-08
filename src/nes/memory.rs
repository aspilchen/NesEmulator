pub trait Memory {
    fn read(&mut self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8);
}