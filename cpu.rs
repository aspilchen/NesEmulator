use crate::{
    adress_mode::{decode_address, AddressMode},
    bus::Bus,
    memory::Memory,
};

use bitmask_enum::bitmask;
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
            stack_ptr: Default::default(),
            status: Status::none(),
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = 2;
        write!(
            f,
            "{:<W$}{:<W$}{:<W$}{:<W$}{:<W$}{:<8} \n{:<W$}{:<W$}{:<W$}{:<W$}{:<W$}{:08b}",
            "A",
            "X",
            "Y",
            "PC",
            "S",
            "P",
            self.accumulator,
            self.index_x,
            self.index_y,
            self.program_counter,
            self.stack_ptr,
            self.status.bits(),
            W = 4
        )
    }
}

impl Cpu {
    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }
}

#[bitmask]
#[bitmask_config(inverted_flags)]
pub enum Status {
    Carry,
    Zero,
    InturruptDisable,
    Decimal,
    B1,
    B2,
    Overflow,
    Negative,
}

impl Status {
    pub fn set_flags(&mut self, flags: Status, is_set: bool) {
        if (is_set) {
            self.bits |= flags.bits;
        } else {
            self.bits &= flags.not().bits;
        }
    }
}
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

pub fn fetch(bus: &mut Bus) -> u8 {
    let address = bus.cpu.program_counter as usize;
    let value = bus.read(address);
    bus.cpu.increment_pc();
    return value;
}

pub fn decode(op_code: u8, bus: &mut Bus) -> Instruction {
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
        _ => panic!("invalid opcode"),
    }
}

pub fn execute(bus: &mut Bus, instruction: &Instruction) {
    match instruction {
        Instruction::ADC(addres_mode) => adc(bus, addres_mode),
        Instruction::AND(address_mode) => and(bus, address_mode),
        Instruction::ASL(address_mode) => asl(bus, address_mode),
        Instruction::BCC(address_mode) => bcc(bus, address_mode),
        Instruction::BCS(address_mode) => bcs(bus, address_mode),
        Instruction::BEQ(address_mode) => beq(bus, address_mode),
        Instruction::BIT(address_mode) => bit(bus, address_mode),

        _ => panic!("invalid instruction"),
    }
}

pub fn adc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
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

pub fn and(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator & param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn asl(bus: &mut Bus, address_mode: &AddressMode) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let address = decode_address(bus, address_mode);
            let temp = bus.read(address);
            (temp, 0)
        }
    };

    let (mut result, is_carry) = param.overflowing_shl(1);
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

pub fn bcc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_carry = bus.cpu.status.contains(Status::Carry);
    if (!is_carry) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bcs(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_carry = bus.cpu.status.contains(Status::Carry);
    if (is_carry) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn beq(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_zero = bus.cpu.status.contains(Status::Zero);
    if (is_zero) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bit(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let accumulator = bus.cpu.accumulator;
    let param = bus.read(address);
    let result = accumulator & param;
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    let is_overflow = (result & 0b01000000) != 0;
    cpu.status.set_flags(Status::Overflow, is_overflow);
}

pub fn bmi(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_negative = bus.cpu.status.contains(Status::Negative);
    if (is_negative) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bne(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_zero = bus.cpu.status.contains(Status::Zero);
    if (!is_zero) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bpl(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_negative = bus.cpu.status.contains(Status::Negative);
    if (!is_negative) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn brk(bus: &mut Bus, address_mode: &AddressMode) {
    todo!("BRK not implemented");
}

pub fn bvc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_overflow = bus.cpu.status.contains(Status::Overflow);
    if (!is_overflow) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bvs(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let is_overflow = bus.cpu.status.contains(Status::Overflow);
    if (is_overflow) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn clc(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status.set_flags(Status::Carry, false);
}

pub fn cld(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status.set_flags(Status::Decimal, false);
}

pub fn cli(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status.set_flags(Status::InturruptDisable, false);
}

pub fn clv(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status.set_flags(Status::Overflow, false);
}

pub fn cmp(bus: &mut Bus, address_mode: &AddressMode) {
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

pub fn cpx(bus: &mut Bus, address_mode: &AddressMode) {
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

pub fn cpy(bus: &mut Bus, address_mode: &AddressMode) {
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

pub fn dec(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result = param.wrapping_sub(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn dex(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_sub(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn dey(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_sub(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn eor(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator ^ param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn inc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let result = param.wrapping_add(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn inx(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_add(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn iny(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_add(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn jmp(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    bus.cpu.program_counter = address as u16;
}

pub fn jsr(bus: &mut Bus, address_mode: &AddressMode) {
    todo!("JSR not implemented");
}

pub fn lda(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.accumulator = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn ldx(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.index_x = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn ldy(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    cpu.index_y = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
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
