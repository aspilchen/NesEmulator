use crate::memory::Memory;

pub struct VRam {
    pub memory: [u8; Self::SIZE],
    pub mirror: VramMirror,
}

impl Default for VRam {
    fn default() -> Self {
        Self {
            memory: [0; Self::SIZE],
            mirror: VramMirror::Horizontal,
        }
    }
}

impl Memory for VRam {
    fn read(&mut self, address: usize) -> u8 {
        let address = self.map_address(address);
        return self.memory[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        let address = self.map_address(address);
        self.memory[address] = value;
    }
}

impl VRam {
    const BEGIN: usize = 0x2000;
    const END: usize = 0x3FFF;
    const SIZE: usize = 0x800;
    const TABLE_SIZE: usize = 0x400;

    fn map_address(&self, address: usize) -> usize {
        let mask = 0x2FFF;
        let address = (address & mask) - Self::BEGIN;
        let result = match self.mirror {
            VramMirror::Vertical => self.map_vertical(address),
            VramMirror::Horizontal => self.map_horizontal(address),
        };
        return result;
    }

    fn map_horizontal(&self, address: usize) -> usize {
        let index = address - Self::BEGIN;
        let table_number = index / Self::TABLE_SIZE;
        const TABLE_A: usize = 0;
        const TABLE_A_MIRROR: usize = 1;
        const TABLE_B: usize = 2;
        const TABLE_B_MIRROR: usize = 3;
        let result = match table_number {
            TABLE_A => 0,
            TABLE_A_MIRROR | TABLE_B => index - Self::TABLE_SIZE,
            TABLE_B_MIRROR => index - (2 * Self::TABLE_SIZE),
            _ => panic!("invalid ppu address at 0x{:04X}", address),
        };
        return result;
    }

    fn map_vertical(&self, address: usize) -> usize {
        let index = address - Self::BEGIN;
        let table_number = index / Self::TABLE_SIZE;
        const TABLE_A: usize = 0;
        const TABLE_A_MIRROR: usize = 2;
        const TABLE_B: usize = 1;
        const TABLE_B_MIRROR: usize = 3;
        let result = match table_number {
            TABLE_A | TABLE_B => index,
            TABLE_A_MIRROR | TABLE_B_MIRROR => index - (2 * Self::TABLE_SIZE),
            _ => panic!("invalid ppu address at 0x{:04X}", address),
        };
        return result;
    }
}

pub enum VramMirror {
    Horizontal,
    Vertical,
}
