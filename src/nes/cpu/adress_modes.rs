use crate::nes::{bus::Bus, memory::Memory};

use super::fetch;

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

pub fn decode_address(bus: &mut Bus, address_mode: &AddressMode) -> (usize, bool) {
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
        _ => panic!("cannot decode address mode {:?}", address_mode),
    }
}

pub fn absolute(bus: &mut Bus) -> (usize, bool) {
    let result = fetch_16(bus) as usize;
    return (result, false);
}

pub fn absolute_x(bus: &mut Bus) -> (usize, bool) {
    let param = fetch_16(bus);
    let index = bus.cpu.index_x as u16;
    let result = param.wrapping_add(index) as usize;
    let is_same_page = pages_match(param as usize, result);
    return (result, is_same_page);
}

pub fn absolute_y(bus: &mut Bus) -> (usize, bool) {
    let param = fetch_16(bus);
    let index = bus.cpu.index_y as u16;
    let result = param.wrapping_add(index) as usize;
    let is_same_page = pages_match(param as usize, result);
    return (result, is_same_page);
}

pub fn immediate(bus: &mut Bus) -> (usize, bool) {
    let result = bus.cpu.program_counter as usize;
    bus.cpu.increment_pc();
    return (result, false);
}

pub fn indirect(bus: &mut Bus) -> (usize, bool) {
    let param = fetch_16(bus) as usize;
    let mask = 0xFF;
    let mut bytes = [0, 0];
    if (param & mask == mask) {
        bytes[0] = bus.read(param);
        bytes[1] = bus.read(param & !mask);
    } else {
        bytes[0] = bus.read(param);
        bytes[1] = bus.read(param + 1);
    }
    let result = u16::from_le_bytes(bytes) as usize;
    return (result, false);
}

pub fn indirect_x(bus: &mut Bus) -> (usize, bool) {
    let param = fetch(bus);
    let index = bus.cpu.index_x;
    let address1 = param.wrapping_add(index);
    let address2 = address1.wrapping_add(1);
    let mut bytes = [0, 0];
    bytes[0] = bus.read(address1 as usize);
    bytes[1] = bus.read(address2 as usize);
    let result = u16::from_le_bytes(bytes) as usize;
    let is_same_page = pages_match(param as usize, result);
    return (result, is_same_page);
}

pub fn indirect_y(bus: &mut Bus) -> (usize, bool) {
    let address1 = fetch(bus);
    let address2 = address1.wrapping_add(1);
    let mut bytes = [0, 0];
    bytes[0] = bus.read(address1 as usize);
    bytes[1] = bus.read(address2 as usize);
    let index = bus.cpu.index_y as u16;
    let address = u16::from_le_bytes(bytes);
    let result = address.wrapping_add(index) as usize;
    let is_same_page = pages_match(address as usize, result);
    // println!("{}", is_same_page);
    return (result, is_same_page);
}

pub fn relative(bus: &mut Bus) -> (usize, bool) {
    let param = fetch(bus) as i8;
    let program_counter = bus.cpu.program_counter as i16;
    let result = program_counter.wrapping_add(param as i16) as usize;
    let is_same_page = pages_match(program_counter as usize, result);
    return (result, is_same_page);
}

pub fn zero_page(bus: &mut Bus) -> (usize, bool) {
    let result = fetch(bus) as usize;
    return (result, false);
}

pub fn zero_page_x(bus: &mut Bus) -> (usize, bool) {
    let param = fetch(bus);
    let index = bus.cpu.index_x;
    let result = param.wrapping_add(index) as usize;
    let is_same_page = pages_match(param as usize, result);
    return (result, is_same_page);
}

pub fn zero_page_y(bus: &mut Bus) -> (usize, bool) {
    let param = fetch(bus);
    let index = bus.cpu.index_y;
    let result = param.wrapping_add(index) as usize;
    let is_same_page = pages_match(param as usize, result);
    return (result, is_same_page);
}

fn fetch_16(bus: &mut Bus) -> u16 {
    let mut bytes = [0, 0];
    bytes[0] = fetch(bus);
    bytes[1] = fetch(bus);
    let result = u16::from_le_bytes(bytes);
    return result;
}

fn pages_match(addr_a: usize, addr_b: usize) -> bool {
    let mask = !0xFF;
    return (addr_a & mask) == (addr_b & mask);
}
