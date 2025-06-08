pub mod adress_modes;
pub mod ricoh6502;
pub mod status;

use super::{bus::Bus, memory::Memory};
use adress_modes::{decode_address, AddressMode};
use ricoh6502::Ricoh6502;
use status::Status;

#[derive(Debug)]
pub enum Instruction {
    ADC(AddressMode, i64),
    AND(AddressMode, i64),
    ASL(AddressMode, i64),
    BCC(AddressMode, i64),
    BCS(AddressMode, i64),
    BEQ(AddressMode, i64),
    BIT(AddressMode, i64),
    BMI(AddressMode, i64),
    BNE(AddressMode, i64),
    BPL(AddressMode, i64),
    BRK(AddressMode, i64),
    BVC(AddressMode, i64),
    BVS(AddressMode, i64),
    CLC(AddressMode, i64),
    CLD(AddressMode, i64),
    CLI(AddressMode, i64),
    CLV(AddressMode, i64),
    CMP(AddressMode, i64),
    CPX(AddressMode, i64),
    CPY(AddressMode, i64),
    DEC(AddressMode, i64),
    DEX(AddressMode, i64),
    DEY(AddressMode, i64),
    EOR(AddressMode, i64),
    INC(AddressMode, i64),
    INX(AddressMode, i64),
    INY(AddressMode, i64),
    JMP(AddressMode, i64),
    JSR(AddressMode, i64),
    LDA(AddressMode, i64),
    LDX(AddressMode, i64),
    LDY(AddressMode, i64),
    LSR(AddressMode, i64),
    NOP(AddressMode, i64),
    ORA(AddressMode, i64),
    PHA(AddressMode, i64),
    PHP(AddressMode, i64),
    PLA(AddressMode, i64),
    PLP(AddressMode, i64),
    ROL(AddressMode, i64),
    ROR(AddressMode, i64),
    RTI(AddressMode, i64),
    RTS(AddressMode, i64),
    SBC(AddressMode, i64),
    SEC(AddressMode, i64),
    SED(AddressMode, i64),
    SEI(AddressMode, i64),
    STA(AddressMode, i64),
    STX(AddressMode, i64),
    STY(AddressMode, i64),
    TAX(AddressMode, i64),
    TAY(AddressMode, i64),
    TSX(AddressMode, i64),
    TXA(AddressMode, i64),
    TXS(AddressMode, i64),
    TYA(AddressMode, i64),
    // unofficial
    AAC(AddressMode, i64),
    AAX(AddressMode, i64),
    ARR(AddressMode, i64),
    ASR(AddressMode, i64),
    ATX(AddressMode, i64),
    AXA(AddressMode, i64),
    AXS(AddressMode, i64),
    DCP(AddressMode, i64),
    DOP(AddressMode, i64),
    ISC(AddressMode, i64),
    KIL(AddressMode, i64),
    LAR(AddressMode, i64),
    LAX(AddressMode, i64),
    RLA(AddressMode, i64),
    RRA(AddressMode, i64),
    SLO(AddressMode, i64),
    SRE(AddressMode, i64),
    SXA(AddressMode, i64),
    SYA(AddressMode, i64),
    TOP(AddressMode, i64),
    XAA(AddressMode, i64),
    XAS(AddressMode, i64),
    // duplicates
    // NOP(AddressMode),
    // SBC(AddressMode),
}

pub fn fetch(bus: &mut Bus) -> u8 {
    let address = bus.cpu.program_counter as usize;
    let value = bus.read(address);
    bus.cpu.increment_pc();
    return value;
}

pub fn decode(op_code: u8, bus: &mut Bus) -> Instruction {
    match op_code {
        0x69 => Instruction::ADC(AddressMode::Immediate, 2),
        0x65 => Instruction::ADC(AddressMode::ZeroPage, 3),
        0x75 => Instruction::ADC(AddressMode::ZeroPageX, 4),
        0x6D => Instruction::ADC(AddressMode::Absolute, 4),
        0x7D => Instruction::ADC(AddressMode::AbsoluteX, 4),
        0x79 => Instruction::ADC(AddressMode::AbsoluteY, 4),
        0x61 => Instruction::ADC(AddressMode::IndirectX, 6),
        0x71 => Instruction::ADC(AddressMode::IndirectY, 5),
        0x29 => Instruction::AND(AddressMode::Immediate, 2),
        0x25 => Instruction::AND(AddressMode::ZeroPage, 3),
        0x35 => Instruction::AND(AddressMode::ZeroPageX, 4),
        0x2D => Instruction::AND(AddressMode::Absolute, 4),
        0x3D => Instruction::AND(AddressMode::AbsoluteX, 4),
        0x39 => Instruction::AND(AddressMode::AbsoluteY, 4),
        0x21 => Instruction::AND(AddressMode::IndirectX, 6),
        0x31 => Instruction::AND(AddressMode::IndirectY, 5),
        0x0A => Instruction::ASL(AddressMode::Accumulator, 2),
        0x06 => Instruction::ASL(AddressMode::ZeroPage, 5),
        0x16 => Instruction::ASL(AddressMode::ZeroPageX, 6),
        0x0E => Instruction::ASL(AddressMode::Absolute, 6),
        0x1E => Instruction::ASL(AddressMode::AbsoluteX, 7),
        0x90 => Instruction::BCC(AddressMode::Relative, 2),
        0xB0 => Instruction::BCS(AddressMode::Relative, 2),
        0xF0 => Instruction::BEQ(AddressMode::Relative, 2),
        0x24 => Instruction::BIT(AddressMode::ZeroPage, 3),
        0x2C => Instruction::BIT(AddressMode::Absolute, 4),
        0x30 => Instruction::BMI(AddressMode::Relative, 2),
        0xD0 => Instruction::BNE(AddressMode::Relative, 2),
        0x10 => Instruction::BPL(AddressMode::Relative, 2),
        0x00 => Instruction::BRK(AddressMode::Implied, 7),
        0x50 => Instruction::BVC(AddressMode::Relative, 2),
        0x70 => Instruction::BVS(AddressMode::Relative, 2),
        0x18 => Instruction::CLC(AddressMode::Implied, 2),
        0xD8 => Instruction::CLD(AddressMode::Implied, 2),
        0x58 => Instruction::CLI(AddressMode::Implied, 2),
        0xB8 => Instruction::CLV(AddressMode::Implied, 2),
        0xC9 => Instruction::CMP(AddressMode::Immediate, 2),
        0xC5 => Instruction::CMP(AddressMode::ZeroPage, 3),
        0xD5 => Instruction::CMP(AddressMode::ZeroPageX, 4),
        0xCD => Instruction::CMP(AddressMode::Absolute, 4),
        0xDD => Instruction::CMP(AddressMode::AbsoluteX, 4),
        0xD9 => Instruction::CMP(AddressMode::AbsoluteY, 4),
        0xC1 => Instruction::CMP(AddressMode::IndirectX, 6),
        0xD1 => Instruction::CMP(AddressMode::IndirectY, 5),
        0xE0 => Instruction::CPX(AddressMode::Immediate, 2),
        0xE4 => Instruction::CPX(AddressMode::ZeroPage, 3),
        0xEC => Instruction::CPX(AddressMode::Absolute, 4),
        0xC0 => Instruction::CPY(AddressMode::Immediate, 2),
        0xC4 => Instruction::CPY(AddressMode::ZeroPage, 3),
        0xCC => Instruction::CPY(AddressMode::Absolute, 4),
        0xC6 => Instruction::DEC(AddressMode::ZeroPage, 5),
        0xD6 => Instruction::DEC(AddressMode::ZeroPageX, 6),
        0xCE => Instruction::DEC(AddressMode::Absolute, 6),
        0xDE => Instruction::DEC(AddressMode::AbsoluteX, 7),
        0xCA => Instruction::DEX(AddressMode::Implied, 2),
        0x88 => Instruction::DEY(AddressMode::Implied, 2),
        0x49 => Instruction::EOR(AddressMode::Immediate, 2),
        0x45 => Instruction::EOR(AddressMode::ZeroPage, 3),
        0x55 => Instruction::EOR(AddressMode::ZeroPageX, 4),
        0x4D => Instruction::EOR(AddressMode::Absolute, 4),
        0x5D => Instruction::EOR(AddressMode::AbsoluteX, 4),
        0x59 => Instruction::EOR(AddressMode::AbsoluteY, 4),
        0x41 => Instruction::EOR(AddressMode::IndirectX, 6),
        0x51 => Instruction::EOR(AddressMode::IndirectY, 5),
        0xE6 => Instruction::INC(AddressMode::ZeroPage, 5),
        0xF6 => Instruction::INC(AddressMode::ZeroPageX, 6),
        0xEE => Instruction::INC(AddressMode::Absolute, 6),
        0xFE => Instruction::INC(AddressMode::AbsoluteX, 7),
        0xE8 => Instruction::INX(AddressMode::Implied, 2),
        0xC8 => Instruction::INY(AddressMode::Implied, 2),
        0x4C => Instruction::JMP(AddressMode::Absolute, 3),
        0x6C => Instruction::JMP(AddressMode::Indirect, 5),
        0x20 => Instruction::JSR(AddressMode::Absolute, 6),
        0xA9 => Instruction::LDA(AddressMode::Immediate, 2),
        0xA5 => Instruction::LDA(AddressMode::ZeroPage, 3),
        0xB5 => Instruction::LDA(AddressMode::ZeroPageX, 4),
        0xAD => Instruction::LDA(AddressMode::Absolute, 4),
        0xBD => Instruction::LDA(AddressMode::AbsoluteX, 4),
        0xB9 => Instruction::LDA(AddressMode::AbsoluteY, 4),
        0xA1 => Instruction::LDA(AddressMode::IndirectX, 6),
        0xB1 => Instruction::LDA(AddressMode::IndirectY, 5),
        0xA2 => Instruction::LDX(AddressMode::Immediate, 2),
        0xA6 => Instruction::LDX(AddressMode::ZeroPage, 3),
        0xB6 => Instruction::LDX(AddressMode::ZeroPageY, 4),
        0xAE => Instruction::LDX(AddressMode::Absolute, 4),
        0xBE => Instruction::LDX(AddressMode::AbsoluteY, 4),
        0xA0 => Instruction::LDY(AddressMode::Immediate, 2),
        0xA4 => Instruction::LDY(AddressMode::ZeroPage, 3),
        0xB4 => Instruction::LDY(AddressMode::ZeroPageX, 4),
        0xAC => Instruction::LDY(AddressMode::Absolute, 4),
        0xBC => Instruction::LDY(AddressMode::AbsoluteX, 4),
        0x4A => Instruction::LSR(AddressMode::Accumulator, 2),
        0x46 => Instruction::LSR(AddressMode::ZeroPage, 5),
        0x56 => Instruction::LSR(AddressMode::ZeroPageX, 6),
        0x4E => Instruction::LSR(AddressMode::Absolute, 6),
        0x5E => Instruction::LSR(AddressMode::AbsoluteX, 7),
        0xEA => Instruction::NOP(AddressMode::Implied, 2),
        0x09 => Instruction::ORA(AddressMode::Immediate, 2),
        0x05 => Instruction::ORA(AddressMode::ZeroPage, 3),
        0x15 => Instruction::ORA(AddressMode::ZeroPageX, 4),
        0x0D => Instruction::ORA(AddressMode::Absolute, 4),
        0x1D => Instruction::ORA(AddressMode::AbsoluteX, 4),
        0x19 => Instruction::ORA(AddressMode::AbsoluteY, 4),
        0x01 => Instruction::ORA(AddressMode::IndirectX, 6),
        0x11 => Instruction::ORA(AddressMode::IndirectY, 5),
        0x48 => Instruction::PHA(AddressMode::Implied, 3),
        0x08 => Instruction::PHP(AddressMode::Implied, 3),
        0x68 => Instruction::PLA(AddressMode::Implied, 4),
        0x28 => Instruction::PLP(AddressMode::Implied, 4),
        0x2A => Instruction::ROL(AddressMode::Accumulator, 2),
        0x26 => Instruction::ROL(AddressMode::ZeroPage, 5),
        0x36 => Instruction::ROL(AddressMode::ZeroPageX, 6),
        0x2E => Instruction::ROL(AddressMode::Absolute, 6),
        0x3E => Instruction::ROL(AddressMode::AbsoluteX, 7),
        0x6A => Instruction::ROR(AddressMode::Accumulator, 2),
        0x66 => Instruction::ROR(AddressMode::ZeroPage, 5),
        0x76 => Instruction::ROR(AddressMode::ZeroPageX, 6),
        0x6E => Instruction::ROR(AddressMode::Absolute, 6),
        0x7E => Instruction::ROR(AddressMode::AbsoluteX, 7),
        0x40 => Instruction::RTI(AddressMode::Implied, 6),
        0x60 => Instruction::RTS(AddressMode::Implied, 6),
        0xE9 => Instruction::SBC(AddressMode::Immediate, 2),
        0xE5 => Instruction::SBC(AddressMode::ZeroPage, 3),
        0xF5 => Instruction::SBC(AddressMode::ZeroPageX, 4),
        0xED => Instruction::SBC(AddressMode::Absolute, 4),
        0xFD => Instruction::SBC(AddressMode::AbsoluteX, 4),
        0xF9 => Instruction::SBC(AddressMode::AbsoluteY, 4),
        0xE1 => Instruction::SBC(AddressMode::IndirectX, 6),
        0xF1 => Instruction::SBC(AddressMode::IndirectY, 5),
        0x38 => Instruction::SEC(AddressMode::Implied, 2),
        0xF8 => Instruction::SED(AddressMode::Implied, 2),
        0x78 => Instruction::SEI(AddressMode::Implied, 2),
        0x85 => Instruction::STA(AddressMode::ZeroPage, 3),
        0x95 => Instruction::STA(AddressMode::ZeroPageX, 4),
        0x8D => Instruction::STA(AddressMode::Absolute, 4),
        0x9D => Instruction::STA(AddressMode::AbsoluteX, 5),
        0x99 => Instruction::STA(AddressMode::AbsoluteY, 5),
        0x81 => Instruction::STA(AddressMode::IndirectX, 6),
        0x91 => Instruction::STA(AddressMode::IndirectY, 6),
        0x86 => Instruction::STX(AddressMode::ZeroPage, 3),
        0x96 => Instruction::STX(AddressMode::ZeroPageY, 4),
        0x8E => Instruction::STX(AddressMode::Absolute, 4),
        0x84 => Instruction::STY(AddressMode::ZeroPage, 3),
        0x94 => Instruction::STY(AddressMode::ZeroPageX, 4),
        0x8C => Instruction::STY(AddressMode::Absolute, 4),
        0xAA => Instruction::TAX(AddressMode::Implied, 2),
        0xA8 => Instruction::TAY(AddressMode::Implied, 2),
        0xBA => Instruction::TSX(AddressMode::Implied, 2),
        0x8A => Instruction::TXA(AddressMode::Implied, 2),
        0x9A => Instruction::TXS(AddressMode::Implied, 2),
        0x98 => Instruction::TYA(AddressMode::Implied, 2),
        // unofficial instructions
        0x0B => Instruction::AAC(AddressMode::Immediate, 2),
        0x2B => Instruction::AAC(AddressMode::Immediate, 2),
        0x87 => Instruction::AAX(AddressMode::ZeroPage, 3),
        0x97 => Instruction::AAX(AddressMode::ZeroPageY, 4),
        0x83 => Instruction::AAX(AddressMode::IndirectX, 6),
        0x8F => Instruction::AAX(AddressMode::Absolute, 4),
        0x6B => Instruction::ARR(AddressMode::Immediate, 2),
        0x4B => Instruction::ASR(AddressMode::Immediate, 2),
        0xAB => Instruction::ATX(AddressMode::Immediate, 2),
        0x9F => Instruction::AXA(AddressMode::AbsoluteY, 2),
        0x93 => Instruction::AXA(AddressMode::IndirectY, 2),
        0xCB => Instruction::AXS(AddressMode::Immediate, 2),
        0xC7 => Instruction::DCP(AddressMode::ZeroPage, 5),
        0xD7 => Instruction::DCP(AddressMode::ZeroPageX, 6),
        0xCF => Instruction::DCP(AddressMode::Absolute, 6),
        0xDF => Instruction::DCP(AddressMode::AbsoluteX, 7),
        0xDB => Instruction::DCP(AddressMode::AbsoluteY, 7),
        0xC3 => Instruction::DCP(AddressMode::IndirectX, 8),
        0xD3 => Instruction::DCP(AddressMode::IndirectY, 8),
        0x04 => Instruction::DOP(AddressMode::ZeroPage, 3),
        0x14 => Instruction::DOP(AddressMode::ZeroPageX, 4),
        0x34 => Instruction::DOP(AddressMode::ZeroPageX, 4),
        0x44 => Instruction::DOP(AddressMode::ZeroPage, 3),
        0x54 => Instruction::DOP(AddressMode::ZeroPageX, 4),
        0x64 => Instruction::DOP(AddressMode::ZeroPage, 3),
        0x74 => Instruction::DOP(AddressMode::ZeroPageX, 4),
        0x80 => Instruction::DOP(AddressMode::Immediate, 2),
        0x82 => Instruction::DOP(AddressMode::Immediate, 2),
        0x89 => Instruction::DOP(AddressMode::Immediate, 2),
        0xC2 => Instruction::DOP(AddressMode::Immediate, 2),
        0xD4 => Instruction::DOP(AddressMode::ZeroPageX, 4),
        0xE2 => Instruction::DOP(AddressMode::Immediate, 2),
        0xF4 => Instruction::DOP(AddressMode::ZeroPageX, 4),
        0xE7 => Instruction::ISC(AddressMode::ZeroPage, 5),
        0xF7 => Instruction::ISC(AddressMode::ZeroPageX, 6),
        0xEF => Instruction::ISC(AddressMode::Absolute, 6),
        0xFF => Instruction::ISC(AddressMode::AbsoluteX, 7),
        0xFB => Instruction::ISC(AddressMode::AbsoluteY, 7),
        0xE3 => Instruction::ISC(AddressMode::IndirectX, 8),
        0xF3 => Instruction::ISC(AddressMode::IndirectY, 8),
        0x02 => Instruction::KIL(AddressMode::Implied, 2),
        0x12 => Instruction::KIL(AddressMode::Implied, 2),
        0x22 => Instruction::KIL(AddressMode::Implied, 2),
        0x32 => Instruction::KIL(AddressMode::Implied, 2),
        0x42 => Instruction::KIL(AddressMode::Implied, 2),
        0x52 => Instruction::KIL(AddressMode::Implied, 2),
        0x62 => Instruction::KIL(AddressMode::Implied, 2),
        0x72 => Instruction::KIL(AddressMode::Implied, 2),
        0x92 => Instruction::KIL(AddressMode::Implied, 2),
        0xB2 => Instruction::KIL(AddressMode::Implied, 2),
        0xD2 => Instruction::KIL(AddressMode::Implied, 2),
        0xF2 => Instruction::KIL(AddressMode::Implied, 2),
        0xBB => Instruction::LAR(AddressMode::AbsoluteY, 2),
        0xA7 => Instruction::LAX(AddressMode::ZeroPage, 3),
        0xB7 => Instruction::LAX(AddressMode::ZeroPageY, 4),
        0xAF => Instruction::LAX(AddressMode::Absolute, 4),
        0xBF => Instruction::LAX(AddressMode::AbsoluteY, 4),
        0xA3 => Instruction::LAX(AddressMode::IndirectX, 6),
        0xB3 => Instruction::LAX(AddressMode::IndirectY, 5),
        0x1A => Instruction::NOP(AddressMode::Implied, 2),
        0x3A => Instruction::NOP(AddressMode::Implied, 2),
        0x5A => Instruction::NOP(AddressMode::Implied, 2),
        0x7A => Instruction::NOP(AddressMode::Implied, 2),
        0xDA => Instruction::NOP(AddressMode::Implied, 2),
        0xFA => Instruction::NOP(AddressMode::Implied, 2),
        0x27 => Instruction::RLA(AddressMode::ZeroPage, 5),
        0x37 => Instruction::RLA(AddressMode::ZeroPageX, 6),
        0x2F => Instruction::RLA(AddressMode::Absolute, 6),
        0x3F => Instruction::RLA(AddressMode::AbsoluteX, 7),
        0x3B => Instruction::RLA(AddressMode::AbsoluteY, 7),
        0x23 => Instruction::RLA(AddressMode::IndirectX, 8),
        0x33 => Instruction::RLA(AddressMode::IndirectY, 8),
        0x67 => Instruction::RRA(AddressMode::ZeroPage, 5),
        0x77 => Instruction::RRA(AddressMode::ZeroPageX, 6),
        0x6F => Instruction::RRA(AddressMode::Absolute, 6),
        0x7F => Instruction::RRA(AddressMode::AbsoluteX, 7),
        0x7B => Instruction::RRA(AddressMode::AbsoluteY, 7),
        0x63 => Instruction::RRA(AddressMode::IndirectX, 8),
        0x73 => Instruction::RRA(AddressMode::IndirectY, 8),
        0xEB => Instruction::SBC(AddressMode::Immediate, 2),
        0x07 => Instruction::SLO(AddressMode::ZeroPage, 5),
        0x17 => Instruction::SLO(AddressMode::ZeroPageX, 6),
        0x0F => Instruction::SLO(AddressMode::Absolute, 6),
        0x1F => Instruction::SLO(AddressMode::AbsoluteX, 7),
        0x1B => Instruction::SLO(AddressMode::AbsoluteY, 7),
        0x03 => Instruction::SLO(AddressMode::IndirectX, 8),
        0x13 => Instruction::SLO(AddressMode::IndirectY, 8),
        0x47 => Instruction::SRE(AddressMode::ZeroPage, 5),
        0x57 => Instruction::SRE(AddressMode::ZeroPageX, 6),
        0x4F => Instruction::SRE(AddressMode::Absolute, 6),
        0x5F => Instruction::SRE(AddressMode::AbsoluteX, 7),
        0x5B => Instruction::SRE(AddressMode::AbsoluteY, 7),
        0x43 => Instruction::SRE(AddressMode::IndirectX, 8),
        0x53 => Instruction::SRE(AddressMode::IndirectY, 8),
        0x9E => Instruction::SXA(AddressMode::AbsoluteY, 2),
        0x9C => Instruction::SYA(AddressMode::AbsoluteX, 2),
        0x0C => Instruction::TOP(AddressMode::Absolute, 4),
        0x1C => Instruction::TOP(AddressMode::AbsoluteX, 4),
        0x3C => Instruction::TOP(AddressMode::AbsoluteX, 4),
        0x5C => Instruction::TOP(AddressMode::AbsoluteX, 4),
        0x7C => Instruction::TOP(AddressMode::AbsoluteX, 4),
        0xDC => Instruction::TOP(AddressMode::AbsoluteX, 4),
        0xFC => Instruction::TOP(AddressMode::AbsoluteX, 4),
        0x8B => Instruction::XAA(AddressMode::Immediate, 2),
        0x9B => Instruction::XAS(AddressMode::Immediate, 2),
        _ => panic!("invalid opcode 0x{:02X}", op_code),
    }
}

pub fn execute(bus: &mut Bus, instruction: &Instruction) {
    match instruction {
        Instruction::ADC(address_mode, n_cycles) => adc(bus, address_mode, n_cycles),
        Instruction::AND(address_mode, n_cycles) => and(bus, address_mode, n_cycles),
        Instruction::ASL(address_mode, n_cycles) => asl(bus, address_mode, n_cycles),
        Instruction::BCC(address_mode, n_cycles) => bcc(bus, address_mode, n_cycles),
        Instruction::BCS(address_mode, n_cycles) => bcs(bus, address_mode, n_cycles),
        Instruction::BEQ(address_mode, n_cycles) => beq(bus, address_mode, n_cycles),
        Instruction::BIT(address_mode, n_cycles) => bit(bus, address_mode, n_cycles),
        Instruction::BMI(address_mode, n_cycles) => bmi(bus, address_mode, n_cycles),
        Instruction::BNE(address_mode, n_cycles) => bne(bus, address_mode, n_cycles),
        Instruction::BPL(address_mode, n_cycles) => bpl(bus, address_mode, n_cycles),
        Instruction::BRK(address_mode, n_cycles) => brk(bus, address_mode, n_cycles),
        Instruction::BVC(address_mode, n_cycles) => bvc(bus, address_mode, n_cycles),
        Instruction::BVS(address_mode, n_cycles) => bvs(bus, address_mode, n_cycles),
        Instruction::CLC(address_mode, n_cycles) => clc(bus, address_mode, n_cycles),
        Instruction::CLD(address_mode, n_cycles) => cld(bus, address_mode, n_cycles),
        Instruction::CLI(address_mode, n_cycles) => cli(bus, address_mode, n_cycles),
        Instruction::CLV(address_mode, n_cycles) => clv(bus, address_mode, n_cycles),
        Instruction::CMP(address_mode, n_cycles) => cmp(bus, address_mode, n_cycles),
        Instruction::CPX(address_mode, n_cycles) => cpx(bus, address_mode, n_cycles),
        Instruction::CPY(address_mode, n_cycles) => cpy(bus, address_mode, n_cycles),
        Instruction::DEC(address_mode, n_cycles) => dec(bus, address_mode, n_cycles),
        Instruction::DEX(address_mode, n_cycles) => dex(bus, address_mode, n_cycles),
        Instruction::DEY(address_mode, n_cycles) => dey(bus, address_mode, n_cycles),
        Instruction::EOR(address_mode, n_cycles) => eor(bus, address_mode, n_cycles),
        Instruction::INC(address_mode, n_cycles) => inc(bus, address_mode, n_cycles),
        Instruction::INX(address_mode, n_cycles) => inx(bus, address_mode, n_cycles),
        Instruction::INY(address_mode, n_cycles) => iny(bus, address_mode, n_cycles),
        Instruction::JMP(address_mode, n_cycles) => jmp(bus, address_mode, n_cycles),
        Instruction::JSR(address_mode, n_cycles) => jsr(bus, address_mode, n_cycles),
        Instruction::LDA(address_mode, n_cycles) => lda(bus, address_mode, n_cycles),
        Instruction::LDX(address_mode, n_cycles) => ldx(bus, address_mode, n_cycles),
        Instruction::LDY(address_mode, n_cycles) => ldy(bus, address_mode, n_cycles),
        Instruction::LSR(address_mode, n_cycles) => lsr(bus, address_mode, n_cycles),
        Instruction::NOP(address_mode, n_cycles) => nop(bus, address_mode, n_cycles),
        Instruction::ORA(address_mode, n_cycles) => ora(bus, address_mode, n_cycles),
        Instruction::PHA(address_mode, n_cycles) => pha(bus, address_mode, n_cycles),
        Instruction::PHP(address_mode, n_cycles) => php(bus, address_mode, n_cycles),
        Instruction::PLA(address_mode, n_cycles) => pla(bus, address_mode, n_cycles),
        Instruction::PLP(address_mode, n_cycles) => plp(bus, address_mode, n_cycles),
        Instruction::ROL(address_mode, n_cycles) => rol(bus, address_mode, n_cycles),
        Instruction::ROR(address_mode, n_cycles) => ror(bus, address_mode, n_cycles),
        Instruction::RTI(address_mode, n_cycles) => rti(bus, address_mode, n_cycles),
        Instruction::RTS(address_mode, n_cycles) => rts(bus, address_mode, n_cycles),
        Instruction::SBC(address_mode, n_cycles) => sbc(bus, address_mode, n_cycles),
        Instruction::SEC(address_mode, n_cycles) => sec(bus, address_mode, n_cycles),
        Instruction::SED(address_mode, n_cycles) => sed(bus, address_mode, n_cycles),
        Instruction::SEI(address_mode, n_cycles) => sei(bus, address_mode, n_cycles),
        Instruction::STA(address_mode, n_cycles) => sta(bus, address_mode, n_cycles),
        Instruction::STX(address_mode, n_cycles) => stx(bus, address_mode, n_cycles),
        Instruction::STY(address_mode, n_cycles) => sty(bus, address_mode, n_cycles),
        Instruction::TAX(address_mode, n_cycles) => tax(bus, address_mode, n_cycles),
        Instruction::TAY(address_mode, n_cycles) => tay(bus, address_mode, n_cycles),
        Instruction::TSX(address_mode, n_cycles) => tsx(bus, address_mode, n_cycles),
        Instruction::TXA(address_mode, n_cycles) => txa(bus, address_mode, n_cycles),
        Instruction::TXS(address_mode, n_cycles) => txs(bus, address_mode, n_cycles),
        Instruction::TYA(address_mode, n_cycles) => tya(bus, address_mode, n_cycles),
        Instruction::AAC(address_mode, n_cycles) => aac(bus, address_mode, n_cycles),
        Instruction::AAX(address_mode, n_cycles) => aax(bus, address_mode, n_cycles),
        Instruction::ARR(address_mode, n_cycles) => arr(bus, address_mode, n_cycles),
        Instruction::ASR(address_mode, n_cycles) => asr(bus, address_mode, n_cycles),
        Instruction::ATX(address_mode, n_cycles) => atx(bus, address_mode, n_cycles),
        Instruction::AXA(address_mode, n_cycles) => axa(bus, address_mode, n_cycles),
        Instruction::AXS(address_mode, n_cycles) => axs(bus, address_mode, n_cycles),
        Instruction::DCP(address_mode, n_cycles) => dcp(bus, address_mode, n_cycles),
        Instruction::DOP(address_mode, n_cycles) => dop(bus, address_mode, n_cycles),
        Instruction::ISC(address_mode, n_cycles) => isc(bus, address_mode, n_cycles),
        Instruction::KIL(address_mode, n_cycles) => kil(bus, address_mode, n_cycles),
        Instruction::LAR(address_mode, n_cycles) => lar(bus, address_mode, n_cycles),
        Instruction::LAX(address_mode, n_cycles) => lax(bus, address_mode, n_cycles),
        Instruction::RLA(address_mode, n_cycles) => rla(bus, address_mode, n_cycles),
        Instruction::RRA(address_mode, n_cycles) => rra(bus, address_mode, n_cycles),
        Instruction::SLO(address_mode, n_cycles) => slo(bus, address_mode, n_cycles),
        Instruction::SRE(address_mode, n_cycles) => sre(bus, address_mode, n_cycles),
        Instruction::SXA(address_mode, n_cycles) => sxa(bus, address_mode, n_cycles),
        Instruction::SYA(address_mode, n_cycles) => sya(bus, address_mode, n_cycles),
        Instruction::TOP(address_mode, n_cycles) => top(bus, address_mode, n_cycles),
        Instruction::XAA(address_mode, n_cycles) => xaa(bus, address_mode, n_cycles),
        Instruction::XAS(address_mode, n_cycles) => xas(bus, address_mode, n_cycles),
        _ => panic!("invalid instruction"),
    }
}

pub fn adc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    adc_helper(bus, param);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn and(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator & param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn asl(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, param) = match address_mode {
        AddressMode::Accumulator => {
            let address = 0;
            let param = bus.cpu.accumulator;
            (address, param)
        }
        _ => {
            let (address, is_same_page) = decode_address(bus, address_mode);
            let param = bus.read(address);
            (address, param)
        }
    };
    const BITMASK: u8 = 0x80;
    let is_carry = (param & BITMASK) != 0;
    let result = param << 1;
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_negative_flag(&mut bus.cpu, result);
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };
    bus.tick(*n_cycles);
}

pub fn bcc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = !bus.cpu.status.contains(Status::Carry);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn bcs(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = bus.cpu.status.contains(Status::Carry);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn beq(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = bus.cpu.status.contains(Status::Zero);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn bit(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let mask = Status::Overflow | Status::Negative;
    let accumulator = bus.cpu.accumulator;
    let param = bus.read(address);
    let result = accumulator & param;
    test_and_set_zero_flag(&mut bus.cpu, result);
    let param_flags = Status::from(param);
    bus.cpu.status = (bus.cpu.status & !mask) | (param_flags & mask);
    bus.tick(*n_cycles);
}

pub fn bmi(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = bus.cpu.status.contains(Status::Negative);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn bne(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = !bus.cpu.status.contains(Status::Zero);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn bpl(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = !bus.cpu.status.contains(Status::Negative);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn brk(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    // todo!("BRK not implemented");
    // let bytes = [bus.read(0xFFFE), bus.read(0xFFFF)];
    // bus.cpu.program_counter = u16::from_le_bytes(bytes);
    bus.tick(*n_cycles);
}

pub fn bvc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = !bus.cpu.status.contains(Status::Overflow);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn bvs(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let is_branch = bus.cpu.status.contains(Status::Overflow);
    branch_helper(bus, address_mode, is_branch);
    bus.tick(*n_cycles);
}

pub fn clc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status &= !Status::Carry;
    bus.tick(*n_cycles);
}

pub fn cld(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status &= !Status::Decimal;
    bus.tick(*n_cycles);
}

pub fn cli(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status &= !Status::InterruptDisable;
    bus.tick(*n_cycles);
}

pub fn clv(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status &= !Status::Overflow;
    bus.tick(*n_cycles);
}

pub fn cmp(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let accumulator = bus.cpu.accumulator;
    compare_helper(bus, accumulator, param);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn cpx(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    // let cpu = &mut bus.cpu;
    let index_x = bus.cpu.index_x;
    compare_helper(bus, index_x, param);
    bus.tick(*n_cycles);
}

pub fn cpy(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let index_y = bus.cpu.index_y;
    compare_helper(bus, index_y, param);
    bus.tick(*n_cycles);
}

pub fn dec(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result = param.wrapping_sub(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
}

pub fn dex(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_sub(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
}

pub fn dey(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_sub(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
}

pub fn eor(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator ^ param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn inc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result = param.wrapping_add(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
}

pub fn inx(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_add(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
}

pub fn iny(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_add(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
}

pub fn jmp(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    bus.cpu.program_counter = address as u16;

    bus.tick(*n_cycles);
}

pub fn jsr(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (param, is_same_page) = decode_address(bus, address_mode);
    let cpu = &mut bus.cpu;
    let bytes = (cpu.program_counter - 1).to_le_bytes();
    cpu.stack_push(bytes[1]);
    cpu.stack_push(bytes[0]);
    bus.cpu.program_counter = param as u16;
    bus.tick(*n_cycles);
}

pub fn lda(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.accumulator = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn ldx(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.index_x = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn ldy(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.index_y = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn lsr(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let (address, is_same_page) = decode_address(bus, address_mode);
            let temp = bus.read(address);
            (temp, address)
        }
    };
    const BITMASK: u8 = 0x01;
    let is_carry = (param & BITMASK) != 0;
    let result = param >> 1;
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    bus.cpu.status &= !Status::Negative;
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };
    bus.tick(*n_cycles);
}

pub fn nop(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.tick(*n_cycles);
}

pub fn ora(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator | param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn pha(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let value = bus.cpu.accumulator;
    bus.cpu.stack_push(value);
    bus.tick(*n_cycles);
}

pub fn php(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let value = (bus.cpu.status | Status::B | Status::Unused).bits();
    bus.cpu.stack_push(value);
    bus.tick(*n_cycles);
}

pub fn pla(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let value = bus.cpu.stack_pop();
    test_and_set_negative_flag(&mut bus.cpu, value);
    test_and_set_zero_flag(&mut bus.cpu, value);
    bus.cpu.accumulator = value;
    bus.tick(*n_cycles);
}

pub fn plp(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let value = bus.cpu.stack_pop();
    let mask = Status::B | Status::Unused;
    let param = Status::from(value);
    let current = bus.cpu.status;
    let result = (current & mask) | (param & !mask);
    bus.cpu.status = result;
    bus.tick(*n_cycles);
}

pub fn rol(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let (address, is_same_page) = decode_address(bus, address_mode);
            let temp = bus.read(address);
            (temp, address)
        }
    };
    let bitmask = 0x80;
    let is_carry = (param & bitmask) != 0;
    let mut result = param << 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result += 1;
    }
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_negative_flag(&mut bus.cpu, result);
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };

    bus.tick(*n_cycles);
}

pub fn ror(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let (address, is_same_page) = decode_address(bus, address_mode);
            let temp = bus.read(address);
            (temp, address)
        }
    };
    let bitmask = 0x01;
    let is_carry = (param & bitmask) != 0;
    let mut result = param >> 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result += 0x80;
    }
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_negative_flag(&mut bus.cpu, result);
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };

    bus.tick(*n_cycles);
}

pub fn rti(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let value = bus.cpu.stack_pop();
    let mask = Status::B | Status::Unused;
    let param = Status::from(value);
    let current = bus.cpu.status;
    bus.cpu.status = (current & mask) | (param & !mask);
    let mut bytes = [0, 0];
    bytes[0] = bus.cpu.stack_pop();
    bytes[1] = bus.cpu.stack_pop();
    bus.cpu.program_counter = u16::from_le_bytes(bytes);
    bus.tick(*n_cycles);
}

pub fn rts(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let mut bytes = [0, 0];
    bytes[0] = bus.cpu.stack_pop();
    bytes[1] = bus.cpu.stack_pop();
    bus.cpu.program_counter = u16::from_le_bytes(bytes) + 1;
    bus.tick(*n_cycles);
}

pub fn sbc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = !bus.read(address);
    adc_helper(bus, param);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn sec(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status |= Status::Carry;
    bus.tick(*n_cycles);
}

pub fn sed(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status |= Status::Decimal;
    bus.tick(*n_cycles);
}

pub fn sei(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    bus.cpu.status |= Status::InterruptDisable;
    bus.tick(*n_cycles);
}

pub fn sta(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let value = bus.cpu.accumulator;
    bus.write(address, value);
    bus.tick(*n_cycles);
}

pub fn stx(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let value = bus.cpu.index_x;
    bus.write(address, value);
    bus.tick(*n_cycles);
}

pub fn sty(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let value = bus.cpu.index_y;
    bus.write(address, value);
    bus.tick(*n_cycles);
}

pub fn tax(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
    bus.tick(*n_cycles);
}

pub fn tay(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_y = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
    bus.tick(*n_cycles);
}

pub fn tsx(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let value = cpu.stack_ptr;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
    bus.tick(*n_cycles);
}

pub fn txa(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
    bus.tick(*n_cycles);
}

pub fn txs(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.stack_ptr = value;
    bus.tick(*n_cycles);
}

pub fn tya(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_y;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
    bus.tick(*n_cycles);
}

// Unofficial instructions

pub fn aac(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let accumulator = bus.cpu.accumulator;
    let result = param & accumulator;
    let is_negative = (result as i8) < 0;
    bus.cpu
        .status
        .set_flags(Status::Negative | Status::Carry, is_negative);
    bus.cpu.status.set_flags(Status::Zero, result == 0);
    bus.tick(*n_cycles);
}

pub fn aax(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let accumulator = bus.cpu.accumulator;
    let index_x = bus.cpu.index_x;
    let result = accumulator & index_x;
    // test_and_set_negative_flag(&mut bus.cpu, result);
    // test_and_set_zero_flag(&mut bus.cpu, result);
    bus.write(address, result);
    bus.tick(*n_cycles);
}

pub fn arr(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let accumulator = bus.cpu.accumulator;
    let result = (accumulator & param) >> 1;
    bus.tick(*n_cycles);
}

pub fn asr(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn atx(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn axa(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn axs(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn dcp(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let (result, _) = param.overflowing_sub(1);
    let accumulator = bus.cpu.accumulator;
    compare_helper(bus, accumulator, result);
    bus.write(address, result);
    bus.tick(*n_cycles);
}

pub fn dop(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (_, is_same_page) = decode_address(bus, address_mode);
    bus.tick(*n_cycles);
}

pub fn isc(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let (param, _) = param.overflowing_add(1);
    adc_helper(bus, !param);
    bus.write(address, param);
    bus.tick(*n_cycles);
}

pub fn kil(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn lar(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn lax(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    bus.cpu.accumulator = param;
    bus.cpu.index_x = param;
    test_and_set_negative_flag(&mut bus.cpu, param);
    test_and_set_zero_flag(&mut bus.cpu, param);
    bus.tick(*n_cycles);
    match address_mode {
        AddressMode::AbsoluteY => cycle_rule_one(bus, address_mode, is_same_page),
        AddressMode::IndirectY => cycle_rule_one(bus, address_mode, is_same_page),
        _ => {}
    }
}

pub fn rla(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let mut result_1 = param << 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result_1 |= 0x01;
    }
    let mask = 0x80;
    bus.cpu.status.set_flags(Status::Carry, param & mask != 0);
    bus.write(address, result_1);
    let accumulator = bus.cpu.accumulator;
    let result_2 = result_1 & accumulator;
    bus.cpu.accumulator = result_2;
    test_and_set_negative_flag(&mut bus.cpu, result_2);
    test_and_set_zero_flag(&mut bus.cpu, result_2);
    bus.tick(*n_cycles);
}

pub fn rra(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let mut result = param >> 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result |= 0x80;
    }
    let mask = 0x01;
    bus.cpu.status.set_flags(Status::Carry, param & mask != 0);
    bus.write(address, result);
    adc_helper(bus, result);
    bus.tick(*n_cycles);
}

pub fn slo(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result_1 = param << 1;
    let mask = 0x80;
    bus.cpu.status.set_flags(Status::Carry, param & mask != 0);
    bus.write(address, result_1);
    let accumulator = bus.cpu.accumulator;
    let result_2 = result_1 | accumulator;
    bus.cpu.accumulator = result_2;
    test_and_set_negative_flag(&mut bus.cpu, result_2);
    test_and_set_zero_flag(&mut bus.cpu, result_2);
    bus.tick(*n_cycles);
}

pub fn sre(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result_1 = param >> 1;
    let mask = 0x01;
    bus.cpu.status.set_flags(Status::Carry, param & mask != 0);
    bus.write(address, result_1);
    let accumulator = bus.cpu.accumulator;
    let result_2 = result_1 ^ accumulator;
    bus.cpu.accumulator = result_2;
    test_and_set_negative_flag(&mut bus.cpu, result_2);
    test_and_set_zero_flag(&mut bus.cpu, result_2);
    bus.tick(*n_cycles);
}

pub fn sxa(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn sya(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn top(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    let (_, is_same_page) = decode_address(bus, address_mode);
    bus.tick(*n_cycles);
    cycle_rule_one(bus, address_mode, is_same_page);
}

pub fn xaa(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

pub fn xas(bus: &mut Bus, address_mode: &AddressMode, n_cycles: &i64) {
    todo!();
    bus.tick(*n_cycles);
}

fn test_and_set_negative_flag(cpu: &mut Ricoh6502, result: u8) {
    let is_negative = (result as i8) < 0;
    cpu.status.set_flags(Status::Negative, is_negative);
}

fn test_and_set_overflow_flag(cpu: &mut Ricoh6502, acc: u8, param: u8, result: u8) {
    let is_overflow = ((result ^ acc) & (result ^ param) & 0x80) != 0;
    cpu.status.set_flags(Status::Overflow, is_overflow);
}

fn test_and_set_zero_flag(cpu: &mut Ricoh6502, result: u8) {
    let is_zero = result == 0;
    cpu.status.set_flags(Status::Zero, is_zero);
}

fn adc_helper(bus: &mut Bus, param: u8) {
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let mut carry2 = false;
    let (mut result, carry1) = accumulator.overflowing_add(param);
    if (cpu.status.contains(Status::Carry)) {
        (result, carry2) = result.overflowing_add(1);
    }
    let is_carry = carry1 || carry2;
    cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_overflow_flag(cpu, accumulator, param, result);
    test_and_set_zero_flag(cpu, result);
    test_and_set_negative_flag(cpu, result);
    cpu.accumulator = result;
}

fn compare_helper(bus: &mut Bus, left: u8, right: u8) {
    let (result, _) = left.overflowing_sub(right);
    bus.cpu.status.set_flags(Status::Carry, left >= right);
    bus.cpu.status.set_flags(Status::Zero, left == right);
    bus.cpu
        .status
        .set_flags(Status::Negative, result & 0x80 != 0);
}

fn branch_helper(bus: &mut Bus, address_mode: &AddressMode, is_branch: bool) {
    let (address, is_same_page) = decode_address(bus, address_mode);
    let mut n_cycles = 0;
    if (is_branch) {
        bus.cpu.program_counter = address as u16;
        n_cycles += 1;
        if (!is_same_page) {
            n_cycles += 1;
        }
    }
    bus.tick(n_cycles);
}

fn cycle_rule_one(bus: &mut Bus, address_mode: &AddressMode, is_same_page: bool) {
    match address_mode {
        AddressMode::AbsoluteX | AddressMode::AbsoluteY | AddressMode::IndirectY => {
            if (!is_same_page) {
                bus.tick(1);
            }
        }
        _ => {}
    }
}
