mod header;

use std::os::raw;

use super::memory::Memory;
use header::{ControlOne, Header};

const HEADER_SIZE: usize = 0x10;
const TRAINER_SIZE: usize = 512;
const PRG_BANK_SIZE: usize = 0x4000;
const CHR_BANK_SIZE: usize = 0x2000;

pub struct Cart {
    pub header: Header,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    // pub ram: Vec<u8>,
    // pub ram_begin: usize,
    // pub ram_end: usize,
    // pub rom_begin: usize,
    // pub rom_end: usize,
}

impl Memory for Cart {
    fn read(&mut self, address: usize) -> u8 {
        todo!();
        // return self.memory[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        todo!();
        // self.memory[address] = value;
    }
}

impl Cart {
    pub const BEGIN: usize = 0x4020;
    pub const END: usize = 0xFFFF;

    pub fn new(raw_data: &Vec<u8>) -> Self {
        let header = Header::new(&raw_data);
        let prg_size = header.num_prg_banks as usize * PRG_BANK_SIZE;
        let chr_size = header.num_chr_banks as usize * CHR_BANK_SIZE;
        let trainer_skip = if (header.control.control_one.contains(ControlOne::Trainer)) {
            TRAINER_SIZE
        } else {
            0
        };
        let prg_begin = HEADER_SIZE + trainer_skip;
        let prg_end = prg_begin + prg_size;
        let chr_begin = prg_end;
        let chr_end = chr_begin + chr_size;
        return Self {
            header: header,
            prg_rom: raw_data[prg_begin..prg_end].to_vec(),
            chr_rom: raw_data[chr_begin..chr_end].to_vec(),
            // ram_end: ram_end,
            // rom_begin: rom_begin,
            // rom_end: rom_end,
        };
    }

    fn map_address(&self, address: usize) -> usize {
        return address - Self::BEGIN;
    }
}
