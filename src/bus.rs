use crate::{
    cartridge::Cartridge,
    cpu::{ram::Ram, Cpu},
    memory::Memory,
    ppu::Ppu,
};

pub struct Bus<Cart: Cartridge + Memory> {
    pub cart: Cart,
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub ram: Ram,
}

impl<Cart: Cartridge + Memory> Memory for Bus<Cart> {
    fn read(&mut self, address: usize) -> u8 {
        let device = self.map_device(address);
        return device.read(address);
    }

    fn write(&mut self, address: usize, value: u8) {
        let device = self.map_device(address);
        device.write(address, value);
    }
}

impl<Cart: Cartridge + Memory> Bus<Cart> {
    pub const CART_BEGIN: usize = 0x4020;
    pub const CART_END: usize = 0xFFFF;

    pub fn new(cart: Cart) -> Self {
        let mut result = Self {
            cart: cart,
            ram: Default::default(),
            cpu: Default::default(),
            ppu: Default::default(),
        };
        result.reset();
        return result;
    }

    pub fn reset(&mut self) {
        let reset_vec = self.cart.get_reset_vector();
        self.cpu = Default::default();
        self.ram = Default::default();
        self.cpu.program_counter = reset_vec;
    }

    fn map_device(&mut self, address: usize) -> &mut dyn Memory {
        match address {
            Ram::BEGIN..=Ram::END => &mut self.ram,
            Self::CART_BEGIN..=Self::CART_END => &mut self.cart,
            Ppu::BEGIN..=Ppu::END | Ppu::DMA => &mut self.ppu,
            _ => panic!("invalid memory access at 0x{:04X}", address),
        }
    }
}
