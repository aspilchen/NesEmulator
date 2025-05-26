pub struct AddressRegister {
    pub bytes: [u8; 2],
    pub latch: usize,
}

impl Default for AddressRegister {
    fn default() -> Self {
        Self {
            bytes: [0, 0],
            latch: 0,
        }
    }
}

impl AddressRegister {
    pub fn set(&mut self, value: u16) {
        self.bytes = (value as u16).to_be_bytes();
    }

    pub fn read(&self) -> u16 {
        return u16::from_be_bytes(self.bytes);
    }

    pub fn write(&mut self, value: u8) {
        self.bytes[self.latch] = value;
        self.latch = (self.latch + 1) % 2;
    }

    pub fn increment(&mut self, value: u16) {
        let result = self.read().wrapping_add(value);
        self.set(result);
    }

    pub fn reset_latch(&mut self) {
        self.latch = 0;
    }
}
