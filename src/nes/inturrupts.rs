use crate::nes::{
    cpu::status::Status,
    ppu::{Ppu, CPU_TO_PPU_CYCLE_SCALE},
};

use super::{bus::Bus, memory::Memory};

const RESET_VECTOR_ADDRESS: [usize; 2] = [0xFFFC, 0xFFFD];
const NMI_VECTOR_ADDRESS: [usize; 2] = [0xFFFA, 0xFFFB];

pub fn reset(bus: &mut Bus) {
    bus.cpu = Default::default();
    bus.ppu = Ppu::new(&bus.cart);
    bus.cpu.program_counter = get_interrupt_vector(bus, InterruptType::Reset);
    bus.ppu.tick(bus.clock * CPU_TO_PPU_CYCLE_SCALE);
}

pub fn nmi(bus: &mut Bus) {
    let bytes = bus.cpu.program_counter.to_le_bytes();
    bus.cpu.stack_push(bytes[1]);
    bus.cpu.stack_push(bytes[0]);
    let status = bus.cpu.status & !Status::B | Status::Unused;
    bus.cpu.stack_push(status.bits());
    bus.cpu.status |= Status::InterruptDisable;
    bus.cpu.program_counter = get_interrupt_vector(bus, InterruptType::NMI);
    bus.tick(2);
}

pub fn brk(bus: &mut Bus) {}

enum InterruptType {
    Reset,
    NMI,
    Break,
}

fn get_interrupt_vector(bus: &mut Bus, interrupt: InterruptType) -> u16 {
    let mut bytes = [0, 0];
    match interrupt {
        InterruptType::Reset => {
            bytes[0] = bus.read(RESET_VECTOR_ADDRESS[0]);
            bytes[1] = bus.read(RESET_VECTOR_ADDRESS[1]);
        }
        InterruptType::NMI => {
            bytes[0] = bus.read(NMI_VECTOR_ADDRESS[0]);
            bytes[1] = bus.read(NMI_VECTOR_ADDRESS[1]);
        }
        _ => panic!(),
    }
    let reset_vector = u16::from_le_bytes(bytes);
    return reset_vector;
}
