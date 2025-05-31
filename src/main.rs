use nes::{cart::Cart, open_ines_file, Nes};

mod nes;

fn main() {
    let cart = open_ines_file("nestest.nes");
    let mut nes = Nes::new(cart);
    nes.run();
}
