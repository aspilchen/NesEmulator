use crate::{cartridge::Cartridge, cpu::Cpu, memory::Memory, ram::Ram};

pub struct Bus<Cart: Cartridge + Memory> {
    pub cpu: Cpu,
    pub ram: Ram,
    pub cart: Cart,

    pub ram_begin: usize,
    pub ram_end: usize,
    pub cart_begin: usize,
    pub cart_end: usize,
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
    pub fn new(cart: Cart) -> Self {
        let ram_begin = 0;
        let ram_end = cart.begin();
        let cart_begin = ram_end;
        let cart_end = cart_begin + cart.get_size();
        let mut result = Self {
            cpu: Default::default(),
            ram: Default::default(),
            cart: cart,
            ram_begin: ram_begin,
            ram_end: ram_end,
            cart_begin: cart_begin,
            cart_end: cart_end,
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
        if (address < self.ram_end) {
            return &mut self.ram;
        } else if (self.cart_begin <= address && address < self.cart_end) {
            return &mut self.cart;
        } else {
            panic!("invalid memory access at 0x{:02X}", address);
        };
    }
}
