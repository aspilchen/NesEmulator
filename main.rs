mod adress_mode;
mod bus;
mod cpu;
mod memory;
mod ram;

use bus::Bus;
use cpu::{decode, execute, fetch, Cpu};

fn main() {
    let mut bus = Bus::default();
    let tmp = -3 as i8;
    bus.ram.memory[0] = 0x90;
    bus.ram.memory[1] = tmp as u8;
    bus.ram.memory[2] = 0x0A;
    bus.ram.memory[3] = 1;
    bus.ram.memory[4] = 0x69;
    bus.ram.memory[5] = 1;

    println!("{}", bus.cpu);
    println!("---");

    let op_code = fetch(&mut bus);
    let instruction = decode(op_code, &mut bus);
    execute(&mut bus, &instruction);
    println!("{0}", bus.cpu);
    println!("---");

    // let op_code = fetch(&mut bus);
    // let instruction = decode(op_code, &mut bus);
    // execute(&mut bus, &instruction);
    // println!("{}", bus.cpu);
    // println!("---");

    // let op_code = fetch(&mut bus);
    // let instruction = decode(op_code, &mut bus);
    // execute(&mut bus, &instruction);
    // println!("{}", bus.cpu);
    // println!("---");
}
