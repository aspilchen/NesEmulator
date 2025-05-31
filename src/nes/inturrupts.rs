use super::{bus::Bus, memory::Memory};

pub fn reset(bus: &mut Bus) {
    let address_one = 0xFFFC;
    let address_two = 0xFFFD;
    let mut bytes = [0, 0];
    bytes[0] = bus.read(address_one);
    bytes[1] = bus.read(address_two);
    let reset_vector = u16::from_le_bytes(bytes);
    bus.cpu = Default::default();
    bus.cpu.program_counter = reset_vector;
}
