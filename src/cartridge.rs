pub trait Cartridge {
    fn get_reset_vector(&self) -> u16;
    fn get_size(&self) -> usize;
    fn begin(&self) -> usize;
    fn get_title(&self) -> String {
        return String::from("");
    }
}

// pub struct Cart {
//     pub memory: Vec<u8>,
// }

// impl Default for Cart {
//     fn default() -> Self {
//         Self {
//             memory: vec![0; Cart::N_BYTES],
//         }
//     }
// }

// impl Memory for Cart {
//     fn read(&mut self, address: usize) -> u8 {
//         let address = Cart::map_address(address);
//         return self.memory[address];
//     }

//     fn write(&mut self, address: usize, value: u8) {
//         let address = Cart::map_address(address);
//         self.memory[address] = value;
//     }
// }

// impl Cart {
//     const N_BYTES: usize = 0xBFE0;
//     const ROM_BEGIN: usize = 0x0600;
//     const ROM_END: usize = 0xFFFF;
//     const PRG_ROM_SIZE: usize = 0x8000;
//     const CHR_ROM_SIZE: usize = 0x2000;

//     pub fn new(raw_data: Vec<u8>) -> Self {
//         return Cart { memory: raw_data };
//     }

//     fn map_address(address: usize) -> usize {
//         if (Cart::ROM_BEGIN <= address && address < Cart::ROM_END) {
//             return address - Cart::ROM_BEGIN;
//         }
//         panic!("invalid memory access");
//     }
// }
