use crate::{
    bus::{self, Bus},
    cartridge::Cartridge,
    cpu::fetch,
    memory::Memory,
};

#[derive(Debug)]
pub enum AddressMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    IndirectIndexed,
}

pub fn decode_address<Cart: Cartridge + Memory>(
    bus: &mut Bus<Cart>,
    address_mode: &AddressMode,
) -> usize {
    match address_mode {
        AddressMode::Absolute => absolute(bus),
        AddressMode::AbsoluteX => absolute_x(bus),
        AddressMode::AbsoluteY => absolute_y(bus),
        // AddressMode::Accumulator => 0,
        AddressMode::Immediate => immediate(bus),
        // AddressMode::Implied => 0,
        AddressMode::Indirect => indirect(bus),
        AddressMode::IndirectX => indirect_x(bus),
        AddressMode::IndirectY => indirect_y(bus),
        AddressMode::ZeroPage => zero_page(bus),
        AddressMode::ZeroPageX => zero_page_x(bus),
        AddressMode::ZeroPageY => zero_page_y(bus),
        AddressMode::Relative => relative(bus),
        _ => panic!("invalid address mode"),
    }
}

pub fn absolute<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let result = fetch_16(bus) as usize;
    return result;
}

pub fn absolute_x<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch_16(bus);
    let index = bus.cpu.index_x as u16;
    let result = param.wrapping_add(index) as usize;
    return result;
}

pub fn absolute_y<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch_16(bus);
    let index = bus.cpu.index_y as u16;
    let result = param.wrapping_add(index) as usize;
    return result;
}

pub fn immediate<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let result = bus.cpu.program_counter as usize;
    bus.cpu.increment_pc();
    return result;
}

pub fn indirect<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch_16(bus) as usize;
    let mut bytes = [0, 0];
    bytes[0] = bus.read(param);
    bytes[1] = bus.read(param + 1);
    let result = u16::from_le_bytes(bytes) as usize;
    return result;
}

pub fn indirect_x<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch(bus);
    let index = bus.cpu.index_x;
    let address1 = param.wrapping_add(index) as u16;
    let address2 = address1.wrapping_add(1);
    let mut bytes = [0, 0];
    bytes[0] = bus.read(address1 as usize);
    bytes[1] = bus.read(address2 as usize);
    let result = u16::from_le_bytes(bytes) as usize;
    return result;
}

pub fn indirect_y<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let address1 = fetch(bus);
    let address2 = address1.wrapping_add(1);
    let mut bytes = [0, 0];
    bytes[0] = bus.read(address1 as usize);
    bytes[1] = bus.read(address2 as usize);
    let index = bus.cpu.index_y as u16;
    let result = u16::from_le_bytes(bytes).wrapping_add(index) as usize;
    return result;
}

pub fn relative<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch(bus) as i8;
    let program_counter = bus.cpu.program_counter as i16;
    let result = program_counter.wrapping_add(param as i16);
    return result as usize;
}

pub fn zero_page<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let result = fetch(bus) as usize;
    return result;
}

pub fn zero_page_x<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch(bus);
    let index = bus.cpu.index_x;
    let result = param.wrapping_add(index) as usize;
    return result;
}

pub fn zero_page_y<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> usize {
    let param = fetch(bus);
    let index = bus.cpu.index_y;
    let result = param.wrapping_add(index) as usize;
    return result;
}

fn fetch_16<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> u16 {
    let mut bytes = [0, 0];
    bytes[0] = fetch(bus);
    bytes[1] = fetch(bus);
    let result = u16::from_le_bytes(bytes);
    return result;
}
