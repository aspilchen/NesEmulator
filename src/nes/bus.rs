use crate::nes::{
    inturrupts::nmi,
    ppu::{Ppu, CPU_TO_PPU_CYCLE_SCALE},
};

use super::{cart::Cart, cpu::ricoh6502::Ricoh6502, inturrupts::reset, memory::Memory};

pub struct Bus {
    pub cart: Cart,
    pub cpu: Ricoh6502,
    pub ppu: Ppu,
    pub clock: i64,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            cart: Default::default(),
            cpu: Default::default(),
            ppu: Default::default(),
            clock: 7,
        }
    }
}

impl Memory for Bus {
    fn read(&mut self, address: usize) -> u8 {
        let device = self.map_device(address);
        return device.read(address);
    }

    fn write(&mut self, address: usize, value: u8) {
        let device = self.map_device(address);
        device.write(address, value);
    }
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        let mut result = Self {
            cart: cart,
            cpu: Default::default(),
            ppu: Default::default(),
            clock: 7,
        };
        reset(&mut result);
        return result;
    }

    fn map_device(&mut self, address: usize) -> &mut dyn Memory {
        match address {
            Ricoh6502::RAM_BEGIN..=Ricoh6502::RAM_END => &mut self.cpu,
            Cart::BEGIN..=Cart::END => &mut self.cart,
            Ppu::BEGIN..=Ppu::END | Ppu::DMA => &mut self.ppu,
            _ => panic!("invalid memory access at 0x{:04X}", address),
        }
    }

    pub fn tick(&mut self, n_cycles: i64) {
        self.clock += n_cycles;
        self.ppu.tick(n_cycles * CPU_TO_PPU_CYCLE_SCALE);
        if (self.ppu.is_nmi_interrupt) {
            nmi(self);
        }
    }
}
