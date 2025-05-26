pub mod ram;
pub mod status;

use crate::{
    adress_mode::{decode_address, AddressMode},
    bus::Bus,
    cartridge::Cartridge,
    memory::Memory,
};

pub use status::Status;
use std::fmt;

pub struct Cpu {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_ptr: u8,
    pub status: Status,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            accumulator: Default::default(),
            index_x: Default::default(),
            index_y: Default::default(),
            program_counter: Default::default(),
            stack_ptr: Self::STACK_PTR_INIT,
            status: Default::default(),
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<W$}{:<W$}{:<W$}{:<W$}{:<4} {:<8} \n{:<W$}{:<W$}{:<W$}{:<W$}{:04X} {:08b}",
            "A",
            "X",
            "Y",
            "S",
            "PC",
            "P",
            self.accumulator,
            self.index_x,
            self.index_y,
            self.stack_ptr,
            self.program_counter,
            self.status.bits(),
            W = 4
        )
    }
}

impl Cpu {
    pub const STACK_PAGE: usize = 0x0100;
    pub const STACK_BOTTOM: usize = 0x1FF;
    pub const STACK_PTR_INIT: u8 = 0xFF;

    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }
}

#[derive(Debug)]
pub enum Instruction {
    ADC(AddressMode),
    AND(AddressMode),
    ASL(AddressMode),
    BCC(AddressMode),
    BCS(AddressMode),
    BEQ(AddressMode),
    BIT(AddressMode),
    BMI(AddressMode),
    BNE(AddressMode),
    BPL(AddressMode),
    BRK(AddressMode),
    BVC(AddressMode),
    BVS(AddressMode),
    CLC(AddressMode),
    CLD(AddressMode),
    CLI(AddressMode),
    CLV(AddressMode),
    CMP(AddressMode),
    CPX(AddressMode),
    CPY(AddressMode),
    DEC(AddressMode),
    DEX(AddressMode),
    DEY(AddressMode),
    EOR(AddressMode),
    INC(AddressMode),
    INX(AddressMode),
    INY(AddressMode),
    JMP(AddressMode),
    JSR(AddressMode),
    LDA(AddressMode),
    LDX(AddressMode),
    LDY(AddressMode),
    LSR(AddressMode),
    NOP(AddressMode),
    ORA(AddressMode),
    PHA(AddressMode),
    PHP(AddressMode),
    PLA(AddressMode),
    PLP(AddressMode),
    ROL(AddressMode),
    ROR(AddressMode),
    RTI(AddressMode),
    RTS(AddressMode),
    SBC(AddressMode),
    SEC(AddressMode),
    SED(AddressMode),
    SEI(AddressMode),
    STA(AddressMode),
    STX(AddressMode),
    STY(AddressMode),
    TAX(AddressMode),
    TAY(AddressMode),
    TSX(AddressMode),
    TXA(AddressMode),
    TXS(AddressMode),
    TYA(AddressMode),
}

pub fn fetch<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> u8 {
    let address = bus.cpu.program_counter as usize;
    let value = bus.read(address);
    bus.cpu.increment_pc();
    return value;
}

pub fn decode<Cart: Cartridge + Memory>(op_code: u8, bus: &mut Bus<Cart>) -> Instruction {
    match op_code {
        0x69 => Instruction::ADC(AddressMode::Immediate),
        0x65 => Instruction::ADC(AddressMode::ZeroPage),
        0x75 => Instruction::ADC(AddressMode::ZeroPageX),
        0x6D => Instruction::ADC(AddressMode::Absolute),
        0x7D => Instruction::ADC(AddressMode::AbsoluteX),
        0x79 => Instruction::ADC(AddressMode::AbsoluteY),
        0x61 => Instruction::ADC(AddressMode::IndirectX),
        0x71 => Instruction::ADC(AddressMode::IndirectY),
        0x29 => Instruction::AND(AddressMode::Immediate),
        0x25 => Instruction::AND(AddressMode::ZeroPage),
        0x35 => Instruction::AND(AddressMode::ZeroPageX),
        0x2D => Instruction::AND(AddressMode::Absolute),
        0x3D => Instruction::AND(AddressMode::AbsoluteX),
        0x39 => Instruction::AND(AddressMode::AbsoluteY),
        0x21 => Instruction::AND(AddressMode::IndirectX),
        0x31 => Instruction::AND(AddressMode::IndirectY),
        0x0A => Instruction::ASL(AddressMode::Accumulator),
        0x06 => Instruction::ASL(AddressMode::ZeroPage),
        0x16 => Instruction::ASL(AddressMode::ZeroPageX),
        0x0E => Instruction::ASL(AddressMode::Absolute),
        0x1E => Instruction::ASL(AddressMode::AbsoluteX),
        0x90 => Instruction::BCC(AddressMode::Relative),
        0xB0 => Instruction::BCS(AddressMode::Relative),
        0xF0 => Instruction::BEQ(AddressMode::Relative),
        0x24 => Instruction::BIT(AddressMode::ZeroPage),
        0x2C => Instruction::BIT(AddressMode::Absolute),
        0x30 => Instruction::BMI(AddressMode::Relative),
        0xD0 => Instruction::BNE(AddressMode::Relative),
        0x10 => Instruction::BPL(AddressMode::Relative),
        0x00 => Instruction::BRK(AddressMode::Implied),
        0x50 => Instruction::BVC(AddressMode::Relative),
        0x70 => Instruction::BVS(AddressMode::Relative),
        0x18 => Instruction::CLC(AddressMode::Implied),
        0xD8 => Instruction::CLD(AddressMode::Implied),
        0x58 => Instruction::CLI(AddressMode::Implied),
        0xB8 => Instruction::CLV(AddressMode::Implied),
        0xC9 => Instruction::CMP(AddressMode::Immediate),
        0xC5 => Instruction::CMP(AddressMode::ZeroPage),
        0xD5 => Instruction::CMP(AddressMode::ZeroPageX),
        0xCD => Instruction::CMP(AddressMode::Absolute),
        0xDD => Instruction::CMP(AddressMode::AbsoluteX),
        0xD9 => Instruction::CMP(AddressMode::AbsoluteY),
        0xC1 => Instruction::CMP(AddressMode::IndirectX),
        0xD1 => Instruction::CMP(AddressMode::IndirectY),
        0xE0 => Instruction::CPX(AddressMode::Immediate),
        0xE4 => Instruction::CPX(AddressMode::ZeroPage),
        0xEC => Instruction::CPX(AddressMode::Absolute),
        0xC0 => Instruction::CPY(AddressMode::Immediate),
        0xC4 => Instruction::CPY(AddressMode::ZeroPage),
        0xCC => Instruction::CPY(AddressMode::Absolute),
        0xC6 => Instruction::DEC(AddressMode::ZeroPage),
        0xD6 => Instruction::DEC(AddressMode::ZeroPageX),
        0xCE => Instruction::DEC(AddressMode::Absolute),
        0xDE => Instruction::DEC(AddressMode::AbsoluteX),
        0xCA => Instruction::DEX(AddressMode::Implied),
        0x88 => Instruction::DEY(AddressMode::Implied),
        0x49 => Instruction::EOR(AddressMode::Immediate),
        0x45 => Instruction::EOR(AddressMode::ZeroPage),
        0x55 => Instruction::EOR(AddressMode::ZeroPageX),
        0x4D => Instruction::EOR(AddressMode::Absolute),
        0x5D => Instruction::EOR(AddressMode::AbsoluteX),
        0x59 => Instruction::EOR(AddressMode::AbsoluteY),
        0x41 => Instruction::EOR(AddressMode::IndirectX),
        0x51 => Instruction::EOR(AddressMode::IndirectY),
        0xE6 => Instruction::INC(AddressMode::ZeroPage),
        0xF6 => Instruction::INC(AddressMode::ZeroPageX),
        0xEE => Instruction::INC(AddressMode::Absolute),
        0xFE => Instruction::INC(AddressMode::AbsoluteX),
        0xE8 => Instruction::INX(AddressMode::Implied),
        0xC8 => Instruction::INY(AddressMode::Implied),
        0x4C => Instruction::JMP(AddressMode::Absolute),
        0x6C => Instruction::JMP(AddressMode::Indirect),
        0x20 => Instruction::JSR(AddressMode::Absolute),
        0xA9 => Instruction::LDA(AddressMode::Immediate),
        0xA5 => Instruction::LDA(AddressMode::ZeroPage),
        0xB5 => Instruction::LDA(AddressMode::ZeroPageX),
        0xAD => Instruction::LDA(AddressMode::Absolute),
        0xBD => Instruction::LDA(AddressMode::AbsoluteX),
        0xB9 => Instruction::LDA(AddressMode::AbsoluteY),
        0xA1 => Instruction::LDA(AddressMode::IndirectX),
        0xB1 => Instruction::LDA(AddressMode::IndirectY),
        0xA2 => Instruction::LDX(AddressMode::Immediate),
        0xA6 => Instruction::LDX(AddressMode::ZeroPage),
        0xB6 => Instruction::LDX(AddressMode::ZeroPageY),
        0xAE => Instruction::LDX(AddressMode::Absolute),
        0xBE => Instruction::LDX(AddressMode::AbsoluteY),
        0xA0 => Instruction::LDY(AddressMode::Immediate),
        0xA4 => Instruction::LDY(AddressMode::ZeroPage),
        0xB4 => Instruction::LDY(AddressMode::ZeroPageX),
        0xAC => Instruction::LDY(AddressMode::Absolute),
        0xBC => Instruction::LDY(AddressMode::AbsoluteX),
        0x4A => Instruction::LSR(AddressMode::Accumulator),
        0x46 => Instruction::LSR(AddressMode::ZeroPage),
        0x56 => Instruction::LSR(AddressMode::ZeroPageX),
        0x4E => Instruction::LSR(AddressMode::Absolute),
        0x5E => Instruction::LSR(AddressMode::AbsoluteX),
        0xEA => Instruction::NOP(AddressMode::Implied),
        0x09 => Instruction::ORA(AddressMode::Immediate),
        0x05 => Instruction::ORA(AddressMode::ZeroPage),
        0x15 => Instruction::ORA(AddressMode::ZeroPageX),
        0x0D => Instruction::ORA(AddressMode::Absolute),
        0x1D => Instruction::ORA(AddressMode::AbsoluteX),
        0x19 => Instruction::ORA(AddressMode::AbsoluteY),
        0x01 => Instruction::ORA(AddressMode::IndirectX),
        0x11 => Instruction::ORA(AddressMode::IndirectY),
        0x48 => Instruction::PHA(AddressMode::Implied),
        0x08 => Instruction::PHP(AddressMode::Implied),
        0x68 => Instruction::PLA(AddressMode::Implied),
        0x28 => Instruction::PLP(AddressMode::Implied),
        0x2A => Instruction::ROL(AddressMode::Accumulator),
        0x26 => Instruction::ROL(AddressMode::ZeroPage),
        0x36 => Instruction::ROL(AddressMode::ZeroPageX),
        0x2E => Instruction::ROL(AddressMode::Absolute),
        0x3E => Instruction::ROL(AddressMode::AbsoluteX),
        0x6A => Instruction::ROR(AddressMode::Accumulator),
        0x66 => Instruction::ROR(AddressMode::ZeroPage),
        0x76 => Instruction::ROR(AddressMode::ZeroPageX),
        0x6E => Instruction::ROR(AddressMode::Absolute),
        0x7E => Instruction::ROR(AddressMode::AbsoluteX),
        0x40 => Instruction::RTI(AddressMode::Implied),
        0x60 => Instruction::RTS(AddressMode::Implied),
        0xE9 => Instruction::SBC(AddressMode::Immediate),
        0xE5 => Instruction::SBC(AddressMode::ZeroPage),
        0xF5 => Instruction::SBC(AddressMode::ZeroPageX),
        0xED => Instruction::SBC(AddressMode::Absolute),
        0xFD => Instruction::SBC(AddressMode::AbsoluteX),
        0xF9 => Instruction::SBC(AddressMode::AbsoluteY),
        0xE1 => Instruction::SBC(AddressMode::IndirectX),
        0xF1 => Instruction::SBC(AddressMode::IndirectY),
        0x38 => Instruction::SEC(AddressMode::Implied),
        0xF8 => Instruction::SED(AddressMode::Implied),
        0x78 => Instruction::SEI(AddressMode::Implied),
        0x85 => Instruction::STA(AddressMode::ZeroPage),
        0x95 => Instruction::STA(AddressMode::ZeroPageX),
        0x8D => Instruction::STA(AddressMode::Absolute),
        0x9D => Instruction::STA(AddressMode::AbsoluteX),
        0x99 => Instruction::STA(AddressMode::AbsoluteY),
        0x81 => Instruction::STA(AddressMode::IndirectX),
        0x91 => Instruction::STA(AddressMode::IndirectY),
        0x86 => Instruction::STX(AddressMode::ZeroPage),
        0x96 => Instruction::STX(AddressMode::ZeroPageY),
        0x8E => Instruction::STX(AddressMode::Absolute),
        0x84 => Instruction::STY(AddressMode::ZeroPage),
        0x94 => Instruction::STY(AddressMode::ZeroPageX),
        0x8C => Instruction::STY(AddressMode::Absolute),
        0xAA => Instruction::TAX(AddressMode::Implied),
        0xA8 => Instruction::TAY(AddressMode::Implied),
        0xBA => Instruction::TSX(AddressMode::Implied),
        0x8A => Instruction::TXA(AddressMode::Implied),
        0x9A => Instruction::TXS(AddressMode::Implied),
        0x98 => Instruction::TYA(AddressMode::Implied),
        _ => panic!("invalid opcode"),
    }
}

pub fn execute<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, instruction: &Instruction) {
    match instruction {
        Instruction::ADC(address_mode) => adc(bus, address_mode),
        Instruction::AND(address_mode) => and(bus, address_mode),
        Instruction::ASL(address_mode) => asl(bus, address_mode),
        Instruction::BCC(address_mode) => bcc(bus, address_mode),
        Instruction::BCS(address_mode) => bcs(bus, address_mode),
        Instruction::BEQ(address_mode) => beq(bus, address_mode),
        Instruction::BIT(address_mode) => bit(bus, address_mode),
        Instruction::BMI(address_mode) => bmi(bus, address_mode),
        Instruction::BNE(address_mode) => bne(bus, address_mode),
        Instruction::BPL(address_mode) => bpl(bus, address_mode),
        Instruction::BRK(address_mode) => brk(bus, address_mode),
        Instruction::BVC(address_mode) => bvc(bus, address_mode),
        Instruction::BVS(address_mode) => bvs(bus, address_mode),
        Instruction::CLC(address_mode) => clc(bus, address_mode),
        Instruction::CLD(address_mode) => cld(bus, address_mode),
        Instruction::CLI(address_mode) => cli(bus, address_mode),
        Instruction::CLV(address_mode) => clv(bus, address_mode),
        Instruction::CMP(address_mode) => cmp(bus, address_mode),
        Instruction::CPX(address_mode) => cpx(bus, address_mode),
        Instruction::CPY(address_mode) => cpy(bus, address_mode),
        Instruction::DEC(address_mode) => dec(bus, address_mode),
        Instruction::DEX(address_mode) => dex(bus, address_mode),
        Instruction::DEY(address_mode) => dey(bus, address_mode),
        Instruction::EOR(address_mode) => eor(bus, address_mode),
        Instruction::INC(address_mode) => inc(bus, address_mode),
        Instruction::INX(address_mode) => inx(bus, address_mode),
        Instruction::INY(address_mode) => iny(bus, address_mode),
        Instruction::JMP(address_mode) => jmp(bus, address_mode),
        Instruction::JSR(address_mode) => jsr(bus, address_mode),
        Instruction::LDA(address_mode) => lda(bus, address_mode),
        Instruction::LDX(address_mode) => ldx(bus, address_mode),
        Instruction::LDY(address_mode) => ldy(bus, address_mode),
        Instruction::LSR(address_mode) => lsr(bus, address_mode),
        Instruction::NOP(address_mode) => nop(bus, address_mode),
        Instruction::ORA(address_mode) => ora(bus, address_mode),
        Instruction::PHA(address_mode) => pha(bus, address_mode),
        Instruction::PHP(address_mode) => php(bus, address_mode),
        Instruction::PLA(address_mode) => pla(bus, address_mode),
        Instruction::PLP(address_mode) => plp(bus, address_mode),
        Instruction::ROL(address_mode) => rol(bus, address_mode),
        Instruction::ROR(address_mode) => ror(bus, address_mode),
        Instruction::RTI(address_mode) => rti(bus, address_mode),
        Instruction::RTS(address_mode) => rts(bus, address_mode),
        Instruction::SBC(address_mode) => sbc(bus, address_mode),
        Instruction::SEC(address_mode) => sec(bus, address_mode),
        Instruction::SED(address_mode) => sed(bus, address_mode),
        Instruction::SEI(address_mode) => sei(bus, address_mode),
        Instruction::STA(address_mode) => sta(bus, address_mode),
        Instruction::STX(address_mode) => stx(bus, address_mode),
        Instruction::STY(address_mode) => sty(bus, address_mode),
        Instruction::TAX(address_mode) => tax(bus, address_mode),
        Instruction::TAY(address_mode) => tay(bus, address_mode),
        Instruction::TSX(address_mode) => tsx(bus, address_mode),
        Instruction::TXA(address_mode) => txa(bus, address_mode),
        Instruction::TXS(address_mode) => txs(bus, address_mode),
        Instruction::TYA(address_mode) => tya(bus, address_mode),
        _ => panic!("invalid instruction"),
    }
}

pub fn adc<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    adc_helper(bus, param);
}

pub fn and<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator & param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn asl<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let (address, param) = match address_mode {
        AddressMode::Accumulator => {
            let address = 0;
            let param = bus.cpu.accumulator;
            (address, param)
        }
        _ => {
            let address = decode_address(bus, address_mode);
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
}

pub fn bcc<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_carry = bus.cpu.status.contains(Status::Carry);
    if (!is_carry) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bcs<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_carry = bus.cpu.status.contains(Status::Carry);
    if (is_carry) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn beq<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_zero = bus.cpu.status.contains(Status::Zero);
    if (is_zero) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bit<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let accumulator = bus.cpu.accumulator;
    let param = bus.read(address);
    let result = accumulator & param;
    let cpu = &mut bus.cpu;
    test_and_set_zero_flag(cpu, result);
    test_and_set_negative_flag(cpu, result);
    let is_overflow = (result & 0b01000000) != 0;
    cpu.status.set_flags(Status::Overflow, is_overflow);
}

pub fn bmi<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_negative = bus.cpu.status.contains(Status::Negative);
    if (is_negative) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bne<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_zero = bus.cpu.status.contains(Status::Zero);
    if (!is_zero) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bpl<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_negative = bus.cpu.status.contains(Status::Negative);
    if (!is_negative) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn brk<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    todo!("BRK not implemented");
}

pub fn bvc<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_overflow = bus.cpu.status.contains(Status::Overflow);
    if (!is_overflow) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bvs<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_overflow = bus.cpu.status.contains(Status::Overflow);
    if (is_overflow) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn clc<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedCarry;
}

pub fn cld<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedDecimal;
}

pub fn cli<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedInturruptDisable;
}

pub fn clv<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedOverflow;
}

pub fn cmp<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let is_carry = accumulator >= param;
    cpu.status.set_flags(Status::Carry, is_carry);
    let is_zero = accumulator == param;
    cpu.status.set_flags(Status::Zero, is_zero);
    let is_negative = accumulator < param;
    cpu.status.set_flags(Status::Negative, is_negative);
}

pub fn cpx<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let index_x = cpu.index_x;
    let is_carry = index_x >= param;
    cpu.status.set_flags(Status::Carry, is_carry);
    let is_zero = index_x == param;
    cpu.status.set_flags(Status::Zero, is_zero);
    let is_negative = index_x < param;
    cpu.status.set_flags(Status::Negative, is_negative);
}

pub fn cpy<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let index_y = cpu.index_y;
    let is_carry = index_y >= param;
    cpu.status.set_flags(Status::Carry, is_carry);
    let is_zero = index_y == param;
    cpu.status.set_flags(Status::Zero, is_zero);
    let is_negative = index_y < param;
    cpu.status.set_flags(Status::Negative, is_negative);
}

pub fn dec<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result = param.wrapping_sub(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn dex<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_sub(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn dey<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_sub(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn eor<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator ^ param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn inc<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result = param.wrapping_add(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn inx<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_add(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn iny<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_add(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn jmp<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    bus.cpu.program_counter = address as u16;
    // todo!("Implement the hardware bug regarding crossing pages");
}

pub fn jsr<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let param = decode_address(bus, address_mode);
    let bytes = bus.cpu.program_counter.wrapping_sub(1).to_le_bytes();
    stack_push(bus, bytes[1]);
    stack_push(bus, bytes[0]);
    bus.cpu.program_counter = param as u16;
}

pub fn lda<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.accumulator = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn ldx<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.index_x = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn ldy<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.index_y = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn lsr<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let address = decode_address(bus, address_mode);
            let temp = bus.read(address);
            (temp, address)
        }
    };
    const BITMASK: u8 = 0x01;
    let is_carry = (param & BITMASK) != 0;
    let result = param >> 1;
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    bus.cpu.status &= Status::InvertedNegative;
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };
}

pub fn nop<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    // todo!("NOP not implemented");
}

pub fn ora<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator | param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn pha<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let value = bus.cpu.accumulator;
    stack_push(bus, value);
}

pub fn php<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let value = bus.cpu.status.bits();
    stack_push(bus, value);
}

pub fn pla<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let value = stack_pop(bus);
    bus.cpu.accumulator = value;
}

pub fn plp<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let value = stack_pop(bus);
    bus.cpu.status = Status::from(value);
}

pub fn rol<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let address = decode_address(bus, address_mode);
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
}

pub fn ror<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let address = decode_address(bus, address_mode);
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
}

pub fn rti<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    todo!("RTI not implemented");
}

pub fn rts<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let mut bytes = [0, 0];
    bytes[0] = stack_pop(bus);
    bytes[1] = stack_pop(bus);
    bus.cpu.program_counter = u16::from_le_bytes(bytes).wrapping_add(1);
}

pub fn sbc<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = !bus.read(address);
    adc_helper(bus, param);
}

pub fn sec<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status |= Status::Carry;
}

pub fn sed<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status |= Status::Decimal;
}

pub fn sei<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    bus.cpu.status |= Status::InturruptDisable;
}

pub fn sta<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let value = bus.cpu.accumulator;
    bus.write(address, value);
}

pub fn stx<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let value = bus.cpu.index_x;
    bus.write(address, value);
}

pub fn sty<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let value = bus.cpu.index_y;
    bus.write(address, value);
}

pub fn tax<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn tay<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_y = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn tsx<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.stack_ptr;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn txa<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn txs<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.stack_ptr = value;
}

pub fn tya<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_y;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

fn test_and_set_negative_flag(cpu: &mut Cpu, result: u8) {
    let is_negative = (result as i8) < 0;
    cpu.status.set_flags(Status::Negative, is_negative);
}

fn test_and_set_overflow_flag(cpu: &mut Cpu, acc: u8, param: u8, result: u8) {
    let is_overflow = ((result ^ acc) & (result ^ param) & 0x80) != 0;
    cpu.status.set_flags(Status::Overflow, is_overflow);
}

fn test_and_set_zero_flag(cpu: &mut Cpu, result: u8) {
    let is_zero = result == 0;
    cpu.status.set_flags(Status::Zero, is_zero);
}

fn stack_push<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, value: u8) {
    let address = Cpu::STACK_PAGE + (bus.cpu.stack_ptr as usize);
    bus.write(address as usize, value);
    bus.cpu.stack_ptr -= 1;
}

fn stack_pop<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>) -> u8 {
    let address = Cpu::STACK_PAGE + (bus.cpu.stack_ptr as usize) + 1;
    let result = bus.read(address as usize);
    bus.cpu.stack_ptr += 1;
    return result;
}

fn adc_helper<Cart: Cartridge + Memory>(bus: &mut Bus<Cart>, param: u8) {
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
