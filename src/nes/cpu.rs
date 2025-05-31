pub mod adress_modes;
pub mod ricoh6502;
pub mod status;

use super::{bus::Bus, memory::Memory};
use adress_modes::{decode_address, AddressMode};
use ricoh6502::Ricoh6502;
use status::Status;

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
    // unofficial
    AAC(AddressMode),
    AAX(AddressMode),
    ARR(AddressMode),
    ASR(AddressMode),
    ATX(AddressMode),
    AXA(AddressMode),
    AXS(AddressMode),
    DCP(AddressMode),
    DOP(AddressMode),
    ISC(AddressMode),
    KIL(AddressMode),
    LAR(AddressMode),
    LAX(AddressMode),
    RLA(AddressMode),
    RRA(AddressMode),
    SLO(AddressMode),
    SRE(AddressMode),
    SXA(AddressMode),
    SYA(AddressMode),
    TOP(AddressMode),
    XAA(AddressMode),
    XAS(AddressMode),
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
        // unofficial instructions
        0x0B => Instruction::AAC(AddressMode::Immediate),
        0x2B => Instruction::AAC(AddressMode::Immediate),
        0x87 => Instruction::AAX(AddressMode::ZeroPage),
        0x97 => Instruction::AAX(AddressMode::ZeroPageY),
        0x83 => Instruction::AAX(AddressMode::IndirectX),
        0x8F => Instruction::AAX(AddressMode::Absolute),
        0x6B => Instruction::ARR(AddressMode::Immediate),
        0x4B => Instruction::ASR(AddressMode::Immediate),
        0xAB => Instruction::ATX(AddressMode::Immediate),
        0x9F => Instruction::AXA(AddressMode::AbsoluteY),
        0x93 => Instruction::AXA(AddressMode::IndirectY),
        0xCB => Instruction::AXS(AddressMode::Immediate),
        0xC7 => Instruction::DCP(AddressMode::ZeroPage),
        0xD7 => Instruction::DCP(AddressMode::ZeroPageX),
        0xCF => Instruction::DCP(AddressMode::Absolute),
        0xDF => Instruction::DCP(AddressMode::AbsoluteX),
        0xDB => Instruction::DCP(AddressMode::AbsoluteY),
        0xC3 => Instruction::DCP(AddressMode::IndirectX),
        0xD3 => Instruction::DCP(AddressMode::IndirectY),
        0x04 => Instruction::DOP(AddressMode::ZeroPage),
        0x14 => Instruction::DOP(AddressMode::ZeroPageX),
        0x34 => Instruction::DOP(AddressMode::ZeroPageX),
        0x44 => Instruction::DOP(AddressMode::ZeroPage),
        0x54 => Instruction::DOP(AddressMode::ZeroPageX),
        0x64 => Instruction::DOP(AddressMode::ZeroPage),
        0x74 => Instruction::DOP(AddressMode::ZeroPageX),
        0x80 => Instruction::DOP(AddressMode::Immediate),
        0x82 => Instruction::DOP(AddressMode::Immediate),
        0x89 => Instruction::DOP(AddressMode::Immediate),
        0xC2 => Instruction::DOP(AddressMode::Immediate),
        0xD4 => Instruction::DOP(AddressMode::ZeroPageX),
        0xE2 => Instruction::DOP(AddressMode::Immediate),
        0xF4 => Instruction::DOP(AddressMode::ZeroPageX),
        0xE7 => Instruction::ISC(AddressMode::ZeroPage),
        0xF7 => Instruction::ISC(AddressMode::ZeroPageX),
        0xEF => Instruction::ISC(AddressMode::Absolute),
        0xFF => Instruction::ISC(AddressMode::AbsoluteX),
        0xFB => Instruction::ISC(AddressMode::AbsoluteY),
        0xE3 => Instruction::ISC(AddressMode::IndirectX),
        0xF3 => Instruction::ISC(AddressMode::IndirectY),
        0x02 => Instruction::KIL(AddressMode::Implied),
        0x12 => Instruction::KIL(AddressMode::Implied),
        0x22 => Instruction::KIL(AddressMode::Implied),
        0x32 => Instruction::KIL(AddressMode::Implied),
        0x42 => Instruction::KIL(AddressMode::Implied),
        0x52 => Instruction::KIL(AddressMode::Implied),
        0x62 => Instruction::KIL(AddressMode::Implied),
        0x72 => Instruction::KIL(AddressMode::Implied),
        0x92 => Instruction::KIL(AddressMode::Implied),
        0xB2 => Instruction::KIL(AddressMode::Implied),
        0xD2 => Instruction::KIL(AddressMode::Implied),
        0xF2 => Instruction::KIL(AddressMode::Implied),
        0xBB => Instruction::LAR(AddressMode::AbsoluteY),
        0xA7 => Instruction::LAX(AddressMode::ZeroPage),
        0xB7 => Instruction::LAX(AddressMode::ZeroPageY),
        0xAF => Instruction::LAX(AddressMode::Absolute),
        0xBF => Instruction::LAX(AddressMode::AbsoluteY),
        0xA3 => Instruction::LAX(AddressMode::IndirectX),
        0xB3 => Instruction::LAX(AddressMode::IndirectY),
        0x1A => Instruction::NOP(AddressMode::Implied),
        0x3A => Instruction::NOP(AddressMode::Implied),
        0x5A => Instruction::NOP(AddressMode::Implied),
        0x7A => Instruction::NOP(AddressMode::Implied),
        0xDA => Instruction::NOP(AddressMode::Implied),
        0xFA => Instruction::NOP(AddressMode::Implied),
        0x27 => Instruction::RLA(AddressMode::ZeroPage),
        0x37 => Instruction::RLA(AddressMode::ZeroPageX),
        0x2F => Instruction::RLA(AddressMode::Absolute),
        0x3F => Instruction::RLA(AddressMode::AbsoluteX),
        0x3B => Instruction::RLA(AddressMode::AbsoluteY),
        0x23 => Instruction::RLA(AddressMode::IndirectX),
        0x33 => Instruction::RLA(AddressMode::IndirectY),
        0x67 => Instruction::RRA(AddressMode::ZeroPage),
        0x77 => Instruction::RRA(AddressMode::ZeroPageX),
        0x6F => Instruction::RRA(AddressMode::Absolute),
        0x7F => Instruction::RRA(AddressMode::AbsoluteX),
        0x7B => Instruction::RRA(AddressMode::AbsoluteY),
        0x63 => Instruction::RRA(AddressMode::IndirectX),
        0x73 => Instruction::RRA(AddressMode::IndirectY),
        0xEB => Instruction::SBC(AddressMode::Immediate),
        0x07 => Instruction::SLO(AddressMode::ZeroPage),
        0x17 => Instruction::SLO(AddressMode::ZeroPageX),
        0x0F => Instruction::SLO(AddressMode::Absolute),
        0x1F => Instruction::SLO(AddressMode::AbsoluteX),
        0x1B => Instruction::SLO(AddressMode::AbsoluteY),
        0x03 => Instruction::SLO(AddressMode::IndirectX),
        0x13 => Instruction::SLO(AddressMode::IndirectY),
        0x47 => Instruction::SRE(AddressMode::ZeroPage),
        0x57 => Instruction::SRE(AddressMode::ZeroPageX),
        0x4F => Instruction::SRE(AddressMode::Absolute),
        0x5F => Instruction::SRE(AddressMode::AbsoluteX),
        0x5B => Instruction::SRE(AddressMode::AbsoluteY),
        0x43 => Instruction::SRE(AddressMode::IndirectX),
        0x53 => Instruction::SRE(AddressMode::IndirectY),
        0x9E => Instruction::SXA(AddressMode::AbsoluteY),
        0x9C => Instruction::SYA(AddressMode::AbsoluteX),
        0x0C => Instruction::TOP(AddressMode::Absolute),
        0x1C => Instruction::TOP(AddressMode::AbsoluteX),
        0x3C => Instruction::TOP(AddressMode::AbsoluteX),
        0x5C => Instruction::TOP(AddressMode::AbsoluteX),
        0x7C => Instruction::TOP(AddressMode::AbsoluteX),
        0xDC => Instruction::TOP(AddressMode::AbsoluteX),
        0xFC => Instruction::TOP(AddressMode::AbsoluteX),
        0x8B => Instruction::XAA(AddressMode::Immediate),
        0x9B => Instruction::XAS(AddressMode::Immediate),
        // _ => panic!("invalid opcode 0x{:02X}", op_code),
    }
}

pub fn execute(bus: &mut Bus, instruction: &Instruction) {
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
        Instruction::AAC(address_mode) => aac(bus, address_mode),
        Instruction::AAX(address_mode) => aax(bus, address_mode),
        Instruction::ARR(address_mode) => arr(bus, address_mode),
        Instruction::ASR(address_mode) => asr(bus, address_mode),
        Instruction::ATX(address_mode) => atx(bus, address_mode),
        Instruction::AXA(address_mode) => axa(bus, address_mode),
        Instruction::AXS(address_mode) => axs(bus, address_mode),
        Instruction::DCP(address_mode) => dcp(bus, address_mode),
        Instruction::DOP(address_mode) => dop(bus, address_mode),
        Instruction::ISC(address_mode) => isc(bus, address_mode),
        Instruction::KIL(address_mode) => kil(bus, address_mode),
        Instruction::LAR(address_mode) => lar(bus, address_mode),
        Instruction::LAX(address_mode) => lax(bus, address_mode),
        Instruction::RLA(address_mode) => rla(bus, address_mode),
        Instruction::RRA(address_mode) => rra(bus, address_mode),
        Instruction::SLO(address_mode) => slo(bus, address_mode),
        Instruction::SRE(address_mode) => sre(bus, address_mode),
        Instruction::SXA(address_mode) => sxa(bus, address_mode),
        Instruction::SYA(address_mode) => sya(bus, address_mode),
        Instruction::TOP(address_mode) => top(bus, address_mode),
        Instruction::XAA(address_mode) => xaa(bus, address_mode),
        Instruction::XAS(address_mode) => xas(bus, address_mode),
        _ => panic!("invalid instruction"),
    }
}

pub fn adc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    adc_helper(bus, param);
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
    let mask = Status::Overflow | Status::Negative;
    let accumulator = bus.cpu.accumulator;
    let param = bus.read(address);
    let result = accumulator & param;
    test_and_set_zero_flag(&mut bus.cpu, result);
    let param_flags = Status::from(param);
    bus.cpu.status = (bus.cpu.status & !mask) | (param_flags & mask);
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
    bus.cpu.status &= Status::InvertedCarry;
}

pub fn cld(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedDecimal;
}

pub fn cli(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedInturruptDisable;
}

pub fn clv(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status &= Status::InvertedOverflow;
}

pub fn cmp(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    compare_helper(cpu, accumulator, param);
}

pub fn cpx(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let index_x = cpu.index_x;
    compare_helper(cpu, index_x, param);
}

pub fn cpy(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let index_y = cpu.index_y;
    compare_helper(cpu, index_y, param);
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
    // todo!("Implement the hardware bug regarding crossing pages");
}

pub fn jsr(bus: &mut Bus, address_mode: &AddressMode) {
    let param = decode_address(bus, address_mode);
    let cpu = &mut bus.cpu;
    let bytes = (cpu.program_counter - 1).to_le_bytes();
    cpu.stack_push(bytes[1]);
    cpu.stack_push(bytes[0]);
    bus.cpu.program_counter = param as u16;
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

pub fn lsr(bus: &mut Bus, address_mode: &AddressMode) {
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

pub fn nop(bus: &mut Bus, address_mode: &AddressMode) {
    // todo!("NOP not implemented");
}

pub fn ora(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator | param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn pha(bus: &mut Bus, address_mode: &AddressMode) {
    let value = bus.cpu.accumulator;
    bus.cpu.stack_push(value);
}

pub fn php(bus: &mut Bus, address_mode: &AddressMode) {
    let value = (bus.cpu.status | Status::B | Status::Unused).bits();
    bus.cpu.stack_push(value);
}

pub fn pla(bus: &mut Bus, address_mode: &AddressMode) {
    let value = bus.cpu.stack_pop();
    test_and_set_negative_flag(&mut bus.cpu, value);
    test_and_set_zero_flag(&mut bus.cpu, value);
    bus.cpu.accumulator = value;
}

pub fn plp(bus: &mut Bus, address_mode: &AddressMode) {
    let value = bus.cpu.stack_pop();
    let mask = Status::B | Status::Unused;
    let param = Status::from(value);
    let current = bus.cpu.status;
    let result = (current & mask) | (param & !mask);
    bus.cpu.status = result;
}

pub fn rol(bus: &mut Bus, address_mode: &AddressMode) {
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

pub fn ror(bus: &mut Bus, address_mode: &AddressMode) {
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

pub fn rti(bus: &mut Bus, address_mode: &AddressMode) {
    let value = bus.cpu.stack_pop();
    let mask = Status::B | Status::Unused;
    let param = Status::from(value);
    let current = bus.cpu.status;
    bus.cpu.status = (current & mask) | (param & !mask);
    let mut bytes = [0, 0];
    bytes[0] = bus.cpu.stack_pop();
    bytes[1] = bus.cpu.stack_pop();
    bus.cpu.program_counter = u16::from_le_bytes(bytes);
}

pub fn rts(bus: &mut Bus, address_mode: &AddressMode) {
    let mut bytes = [0, 0];
    bytes[0] = bus.cpu.stack_pop();
    bytes[1] = bus.cpu.stack_pop();
    bus.cpu.program_counter = u16::from_le_bytes(bytes) + 1;
}

pub fn sbc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = !bus.read(address);
    adc_helper(bus, param);
}

pub fn sec(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status |= Status::Carry;
}

pub fn sed(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status |= Status::Decimal;
}

pub fn sei(bus: &mut Bus, address_mode: &AddressMode) {
    bus.cpu.status |= Status::InturruptDisable;
}

pub fn sta(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let value = bus.cpu.accumulator;
    bus.write(address, value);
}

pub fn stx(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let value = bus.cpu.index_x;
    bus.write(address, value);
}

pub fn sty(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let value = bus.cpu.index_y;
    bus.write(address, value);
}

pub fn tax(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn tay(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_y = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn tsx(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.stack_ptr;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn txa(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn txs(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.stack_ptr = value;
}

pub fn tya(bus: &mut Bus, address_mode: &AddressMode) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_y;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

// Unofficial instructions

pub fn aac(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let accumulator = bus.cpu.accumulator;
    let result = param & accumulator;
    let is_negative = (result as i8) < 0;
    bus.cpu
        .status
        .set_flags(Status::Negative | Status::Carry, is_negative);
    bus.cpu.status.set_flags(Status::Zero, result == 0);
}

pub fn aax(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let accumulator = bus.cpu.accumulator;
    let index_x = bus.cpu.index_x;
    let result = accumulator & index_x;
    // test_and_set_negative_flag(&mut bus.cpu, result);
    // test_and_set_zero_flag(&mut bus.cpu, result);
    bus.write(address, result);
}

pub fn arr(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let accumulator = bus.cpu.accumulator;
    let result = (accumulator & param) >> 1;
}

pub fn asr(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn atx(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn axa(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn axs(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn dcp(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let (result, _) = param.overflowing_sub(1);
    let accumulator = bus.cpu.accumulator;
    compare_helper(&mut bus.cpu, accumulator, result);
    bus.write(address, result);
}

pub fn dop(bus: &mut Bus, address_mode: &AddressMode) {
    let _ = decode_address(bus, address_mode);
}

pub fn isc(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let (param, _) = param.overflowing_add(1);
    adc_helper(bus, !param);
    bus.write(address, param);
}

pub fn kil(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn lar(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn lax(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    bus.cpu.accumulator = param;
    bus.cpu.index_x = param;
    test_and_set_negative_flag(&mut bus.cpu, param);
    test_and_set_zero_flag(&mut bus.cpu, param);
}

pub fn rla(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
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
}

pub fn rra(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
    let param = bus.read(address);
    let mut result = param >> 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result |= 0x80;
    }
    let mask = 0x01;
    bus.cpu.status.set_flags(Status::Carry, param & mask != 0);
    bus.write(address, result);
    adc_helper(bus, result);
}

pub fn slo(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
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
}

pub fn sre(bus: &mut Bus, address_mode: &AddressMode) {
    let address = decode_address(bus, address_mode);
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
}

pub fn sxa(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn sya(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn top(bus: &mut Bus, address_mode: &AddressMode) {
    let _ = decode_address(bus, address_mode);
}

pub fn xaa(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
}

pub fn xas(bus: &mut Bus, address_mode: &AddressMode) {
    todo!();
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

fn compare_helper(cpu: &mut Ricoh6502, left: u8, right: u8) {
    let (result, _) = left.overflowing_sub(right);
    cpu.status.set_flags(Status::Carry, left >= right);
    cpu.status.set_flags(Status::Zero, left == right);
    cpu.status.set_flags(Status::Negative, result & 0x80 != 0);
}
