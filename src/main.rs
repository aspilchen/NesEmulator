mod nes;

use std::fs::File;
use std::io::{self, Read};

use nes::cart::Cart;

fn main() {
    let mut file = File::open("nestest.nes").expect("");
    let mut raw_data = Vec::new();
    file.read_to_end(&mut raw_data);
    let cart = Cart::new(&raw_data);
    println!("{:X}", cart.prg_rom[0]);
}
