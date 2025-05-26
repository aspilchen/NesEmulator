use super::{cart::Cart, cpu::ricoh6502::Ricoh6502, memory::Memory};

pub struct Bus {
    pub cart: Cart,
    pub cpu: Ricoh6502,
    // pub ppu: Ppu,
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
    pub const CART_BEGIN: usize = 0x4020;
    pub const CART_END: usize = 0xFFFF;

    pub fn new(cart: Cart) -> Self {
        let mut result = Self {
            cart: cart,
            cpu: Default::default(),
            // ppu: Default::default(),
        };
        // result.reset();
        return result;
    }

    // pub fn reset(&mut self) {
    //     let reset_vec = self.cart.get_reset_vector();
    //     self.cpu = Default::default();
    //     self.cpu.program_counter = reset_vec;
    // }

    fn map_device(&mut self, address: usize) -> &mut dyn Memory {
        match address {
            Ricoh6502::RAM_BEGIN..=Ricoh6502::RAM_END => &mut self.cpu,
            Self::CART_BEGIN..=Self::CART_END => &mut self.cart,
            // Ppu::BEGIN..=Ppu::END | Ppu::DMA => &mut self.ppu,
            _ => panic!("invalid memory access at 0x{:04X}", address),
        }
    }
}
