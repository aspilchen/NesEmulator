use std::{fs::File, io::Read};

use bus::Bus;
use cart::Cart;
use cpu::{decode, execute, fetch};
use inturrupts::reset;

// mod apu;
mod bus;
pub mod cart;
mod cpu;
pub mod memory;
// mod ppu;
// mod test;
mod inturrupts;

pub struct Nes {
    bus: Bus,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            bus: Default::default(),
        }
    }
}

impl Nes {
    pub fn new(cart: Cart) -> Self {
        Self {
            bus: Bus::new(cart),
        }
    }

    pub fn tick(&mut self) {
        let pc = self.bus.cpu.program_counter;
        let a = self.bus.cpu.accumulator;
        let x = self.bus.cpu.index_x;
        let y = self.bus.cpu.index_y;
        let p = self.bus.cpu.status;
        let sp = self.bus.cpu.stack_ptr;
        let op_code = fetch(&mut self.bus);
        let instruction = decode(op_code, &mut self.bus);
        let op_string = format!("{:?}", instruction);
        println!(
            "{:04X} {:02X} {:<20} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            pc, op_code, op_string, a, x, y, p, sp
        );
        execute(&mut self.bus, &instruction);
    }

    pub fn reset(&mut self) {
        reset(&mut self.bus);
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        }
    }
}

pub fn open_ines_file(filename: &str) -> Cart {
    let mut file = File::open(filename).expect("file not found");
    let mut raw_data = Vec::new();
    file.read_to_end(&mut raw_data);
    let result = Cart::new(&raw_data);
    return result;
}
