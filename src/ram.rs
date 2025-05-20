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
    const N_BYTES: usize = 2048;

    pub fn map_address(address: usize) -> usize {
        let result = if (address < Ram::N_BYTES) {
            address
        } else if (Ram::N_BYTES <= address && address < Ram::N_BYTES * 2) {
            address - Ram::N_BYTES
        } else {
            panic!("invalid memory access {:04X}", address)
        };
        return result;
    }
}
