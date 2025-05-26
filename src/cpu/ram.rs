use crate::memory::Memory;

pub struct Ram {
    pub memory: [u8; Ram::N_BYTES],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            memory: [0; Ram::N_BYTES],
        }
    }
}

impl Memory for Ram {
    fn read(&mut self, address: usize) -> u8 {
        let address = Ram::map_address(address);
        let result = self.memory[address];
        return result;
    }

    fn write(&mut self, address: usize, value: u8) {
        let address = Ram::map_address(address);
        self.memory[address] = value;
    }
}

impl Ram {
    pub const N_BYTES: usize = 2048;
    pub const BEGIN: usize = 0;
    pub const END: usize = 0x1FFF;
    const MIRROR_MASK: usize = 0x7FF;

    pub fn map_address(address: usize) -> usize {
        return address & Self::MIRROR_MASK;
    }
}
