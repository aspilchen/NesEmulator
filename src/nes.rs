use std::{fs::File, io::Read, thread::sleep};

use bus::Bus;
use cart::Cart;
use cpu::{decode, execute, fetch};
use inturrupts::reset;
use std::{thread, time};

// mod apu;
mod bus;
pub mod cart;
mod cpu;
pub mod memory;
// mod ppu;
// mod test;
mod inturrupts;

const CYCLES_PER_SECOND: f64 = 1790000.0;
const WAIT_TIME: f64 = 1000.0 / CYCLES_PER_SECOND;

pub struct Nes {
    bus: Bus,
    wait_time: time::Duration,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            bus: Default::default(),
            wait_time: time::Duration::from_secs_f64(WAIT_TIME),
        }
    }
}

impl Nes {
    pub fn new(cart: Cart) -> Self {
        Self {
            bus: Bus::new(cart),
            wait_time: time::Duration::from_secs_f64(WAIT_TIME),
        }
    }

    pub fn tick(&mut self) {
        if (self.bus.clock <= 0) {
            let op_code = fetch(&mut self.bus);
            let instruction = decode(op_code, &mut self.bus);
            execute(&mut self.bus, &instruction);
        } else {
            self.bus.clock -= 1;
        }
    }

    pub fn tick_debug(&mut self) {
        let pc = self.bus.cpu.program_counter;
        let a = self.bus.cpu.accumulator;
        let x = self.bus.cpu.index_x;
        let y = self.bus.cpu.index_y;
        let p = self.bus.cpu.status;
        let sp = self.bus.cpu.stack_ptr;
        let clock = self.bus.clock;
        let op_code = fetch(&mut self.bus);
        let instruction = decode(op_code, &mut self.bus);
        let op_string = format!("{:?}", instruction);
        execute(&mut self.bus, &instruction);
        println!(
            "{:04X} {:02X} {:<20} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            pc, op_code, op_string, a, x, y, p, sp, clock,
        );
        // self.clock += cycles;
    }

    pub fn reset(&mut self) {
        reset(&mut self.bus);
    }

    pub fn run(&mut self) {
        loop {
            sleep(self.wait_time);
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
