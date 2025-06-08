use crate::nes::memory::Memory;

use super::status::Status;
use std::fmt;

const STACK_PAGE: usize = 0x100;

#[derive(Debug)]
pub struct Ricoh6502 {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_ptr: u8,
    pub status: Status,
    pub ram: [u8; Self::RAM_SIZE],
}

impl Default for Ricoh6502 {
    fn default() -> Self {
        Self {
            accumulator: Default::default(),
            index_x: Default::default(),
            index_y: Default::default(),
            program_counter: Default::default(),
            stack_ptr: Self::STACK_PTR_INIT,
            status: Default::default(),
            ram: [0; Self::RAM_SIZE],
        }
    }
}

impl fmt::Display for Ricoh6502 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<W$}{:<W$}{:<W$}{:<W$}{:<4} {:<8} \n{:<W$}{:<W$}{:<W$}{:<W$}{:04X} {:08b}",
            "A",
            "X",
            "Y",
            "S",
            "PC",
            "P",
            self.accumulator,
            self.index_x,
            self.index_y,
            self.stack_ptr,
            self.program_counter,
            self.status.bits(),
            W = 4
        )
    }
}

impl Memory for Ricoh6502 {
    fn read(&mut self, address: usize) -> u8 {
        let address = self.map_address(address);
        return self.ram[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        let address = self.map_address(address);
        self.ram[address] = value;
    }
}

impl Ricoh6502 {
    pub const RAM_BEGIN: usize = 0;
    pub const RAM_END: usize = 0x1FFF;
    pub const RAM_SIZE: usize = 0x800;
    pub const STACK_PTR_INIT: u8 = 0xFD;

    fn map_address(&self, address: usize) -> usize {
        let mirror_mask = 0x7FF;
        return address & mirror_mask;
    }

    pub fn stack_push(&mut self, value: u8) {
        let address = STACK_PAGE + self.stack_ptr as usize;
        self.stack_ptr -= 1;
        self.ram[address] = value;
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_ptr += 1;
        let address = STACK_PAGE + self.stack_ptr as usize;
        return self.ram[address];
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }
}
