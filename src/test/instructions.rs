use crate::{cartridge::Cartridge, memory::Memory};

struct TestCart {
    pub mem: Vec<u8>,
}

impl Memory for TestCart {
    fn read(&mut self, address: usize) -> u8 {
        let address = Self::map_address(address);
        return self.mem[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        // println!("write {} at {}", value, address);
        let address = Self::map_address(address);
        self.mem[address] = value;
    }
}

impl Cartridge for TestCart {
    fn get_reset_vector(&self) -> u16 {
        return 0x8000;
    }

    fn get_size(&self) -> usize {
        return self.mem.len();
    }

    fn begin(&self) -> usize {
        return 0x8000;
    }
}

impl Default for TestCart {
    fn default() -> Self {
        Self { mem: vec![0; 1024] }
    }
}

impl TestCart {
    pub const BEGIN: usize = 0x8000;

    pub fn new(mem: Vec<u8>) -> Self {
        return Self { mem: mem };
    }

    fn map_address(address: usize) -> usize {
        return address - Self::BEGIN;
    }
}

#[cfg(test)]
mod test_instructions {
    use crate::{
        adress_mode::AddressMode,
        bus::Bus,
        cpu::{execute, Cpu, Instruction, Status},
        test::instructions,
    };

    use super::*;

    #[test]
    fn adc() {
        let param = 0x01;
        let test_code = vec![0x01, 0x7F, 0x80, 0x01];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::ADC(AddressMode::Immediate);
        let base_status = Status::default();
        bus.reset();
        {
            // Basic addition
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 1);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            // Check overflow and negative
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80);
            assert_eq!(
                bus.cpu.status,
                base_status | Status::Overflow | Status::Negative
            )
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(
                bus.cpu.status,
                base_status | Status::Carry | Status::Zero | Status::Overflow
            );
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 2);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn and() {
        let test_code = vec![0xF1, 0x01, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::AND(AddressMode::Immediate);
        let base_status = Status::default();
        bus.reset();
        bus.cpu.accumulator = 0xFF;
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0xF1);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x01);
            assert_eq!(bus.cpu.status, base_status)
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x00);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
    }

    #[test]
    fn asl() {
        let test_code = vec![0x02, 0x01, 0x02];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::ASL(AddressMode::Accumulator);
        let base_status = Status::default();
        bus.cpu.accumulator = 0x80 >> 2;
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80 >> 1);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Carry | Status::Zero);
        }
        {
            let instruction = Instruction::ASL(AddressMode::ZeroPage);
            bus.ram.memory[2] = 2;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[2], 4);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn bcc() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BCC(AddressMode::Relative);
        bus.reset();
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status |= Status::Carry;
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn bcs() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BCS(AddressMode::Relative);
        bus.reset();
        bus.cpu.status |= Status::Carry;
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status = Status::default();
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn beq() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BEQ(AddressMode::Relative);
        bus.reset();
        bus.cpu.status |= Status::Zero;
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status = Status::default();
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn bit() {
        let test_code = vec![0x00, 0x80 >> 1, 0xFF, 0x02];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BIT(AddressMode::Immediate);
        bus.reset();
        bus.cpu.accumulator = 0xFF;
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Overflow);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(
                bus.cpu.status,
                base_status | Status::Overflow | Status::Negative
            );
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
        }
        assert_eq!(bus.cpu.accumulator, 0xFF);
    }

    #[test]
    fn bmi() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BMI(AddressMode::Relative);
        bus.reset();
        bus.cpu.status |= Status::Negative;
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status = Status::default();
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn bne() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BNE(AddressMode::Relative);
        bus.reset();
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status |= Status::Zero;
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn bpl() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BPL(AddressMode::Relative);
        bus.reset();
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status |= Status::Negative;
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn brk() {}

    #[test]
    fn bvc() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BVC(AddressMode::Relative);
        bus.reset();
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status |= Status::Overflow;
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn bvs() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::BVS(AddressMode::Relative);
        bus.reset();
        bus.cpu.status |= Status::Overflow;
        {
            execute(&mut bus, &instruction);
        }
        {
            execute(&mut bus, &instruction);
        }
        {
            bus.cpu.status = Status::default();
            execute(&mut bus, &instruction);
        }
        assert_eq!(bus.cpu.program_counter, 0x02 + TestCart::BEGIN as u16);
    }

    #[test]
    fn clc() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CLC(AddressMode::Implied);
        let base_status = Status::default();
        bus.reset();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.status = Status::all_flags();
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, Status::InvertedCarry);
        }
    }

    #[test]
    fn cld() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CLD(AddressMode::Implied);
        let base_status = Status::default();
        bus.reset();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.status = Status::all_flags();
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, Status::InvertedDecimal);
        }
    }

    #[test]
    fn cli() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CLI(AddressMode::Implied);
        let base_status = Status::default();
        bus.reset();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.status = Status::all_flags();
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, Status::InvertedInturruptDisable);
        }
    }

    #[test]
    fn clv() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CLV(AddressMode::Implied);
        let base_status = Status::default();
        bus.reset();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.status = Status::all_flags();
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, Status::InvertedOverflow);
        }
    }

    #[test]
    fn cmp() {
        let param = 0x10;
        let test_code = vec![param];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CMP(AddressMode::Immediate);
        let base_status = Status::default();
        {
            bus.reset();
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Negative;
            assert_eq!(bus.cpu.status, expected);
        }
        {
            bus.reset();
            bus.cpu.accumulator = param;
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Zero | Status::Carry;
            assert_eq!(bus.cpu.status, expected);
        }
        {
            bus.reset();
            bus.cpu.accumulator = param + 1;
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Carry;
            assert_eq!(bus.cpu.status, expected);
        }
    }

    #[test]
    fn cpx() {
        let param = 0x10;
        let test_code = vec![param];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CPX(AddressMode::Immediate);
        let base_status = Status::default();
        {
            bus.reset();
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Negative;
            assert_eq!(bus.cpu.status, expected);
        }
        {
            bus.reset();
            bus.cpu.index_x = param;
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Zero | Status::Carry;
            assert_eq!(bus.cpu.status, expected);
        }
        {
            bus.reset();
            bus.cpu.index_x = param + 1;
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Carry;
            assert_eq!(bus.cpu.status, expected);
        }
    }

    #[test]
    fn cpy() {
        let param = 0x10;
        let test_code = vec![param];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::CPY(AddressMode::Immediate);
        let base_status = Status::default();
        {
            bus.reset();
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Negative;
            assert_eq!(bus.cpu.status, expected);
        }
        {
            bus.reset();
            bus.cpu.index_y = param;
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Zero | Status::Carry;
            assert_eq!(bus.cpu.status, expected);
        }
        {
            bus.reset();
            bus.cpu.index_y = param + 1;
            execute(&mut bus, &instruction);
            let expected = base_status | Status::Carry;
            assert_eq!(bus.cpu.status, expected);
        }
    }

    #[test]
    fn dec() {
        let test_code = vec![0x10, 0x01, 0xFF];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::DEC(AddressMode::Immediate);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cart.mem[0], 0x0F);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cart.mem[1], 0x00);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cart.mem[2], 0xFE);
        }
    }

    #[test]
    fn dex() {
        let test_code = vec![0x10, 0x01, 0xFF];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::DEX(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.index_x = 0x10;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cpu.index_x, 0x0F);
        }
        {
            bus.cpu.index_x = 0x01;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cpu.index_x, 0x00);
        }
        {
            bus.cpu.index_x = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cpu.index_x, 0xFE);
        }
    }

    #[test]
    fn dey() {
        let test_code = vec![0x10, 0x01, 0xFF];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::DEY(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.index_y = 0x10;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cpu.index_y, 0x0F);
        }
        {
            bus.cpu.index_y = 0x01;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cpu.index_y, 0x00);
        }
        {
            bus.cpu.index_y = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cpu.index_y, 0xFE);
        }
    }

    #[test]
    fn eor() {
        let test_code = vec![0b10101010];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::EOR(AddressMode::Immediate);
        let base_status = Status::default();
        bus.reset();
        {
            bus.reset();
            bus.cpu.accumulator = 0b10101011;
            bus.cpu.status |= Status::Negative | Status::Zero;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cpu.accumulator, 1);
        }
        {
            bus.reset();
            bus.cpu.accumulator = 0b10101010;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cpu.accumulator, 0x00);
        }
        {
            bus.reset();
            bus.cpu.accumulator = 0b00101011;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cpu.accumulator, 0b10000001);
        }
    }

    #[test]
    fn inc() {
        let test_code = vec![0x10, 0xFF, 0xFD];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::INC(AddressMode::Immediate);
        let base_status = Status::default();
        {
            bus.cpu.status |= Status::Zero | Status::Negative;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cart.mem[0], 0x11);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cart.mem[1], 0x00);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cart.mem[2], 0xFE);
        }
    }

    #[test]
    fn inx() {
        let test_code = vec![0x10, 0x01, 0xFF];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::INX(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.index_x = 0x0E;
            bus.cpu.status |= Status::Zero | Status::Negative;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cpu.index_x, 0x0F);
        }
        {
            bus.cpu.index_x = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cpu.index_x, 0x00);
        }
        {
            bus.cpu.index_x = 0xFD;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cpu.index_x, 0xFE);
        }
    }

    #[test]
    fn iny() {
        let test_code = vec![0x10, 0x01, 0xFF];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::INY(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.index_y = 0x0E;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status);
            assert_eq!(bus.cpu.index_y, 0x0F);
        }
        {
            bus.cpu.index_y = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
            assert_eq!(bus.cpu.index_y, 0x00);
        }
        {
            bus.cpu.index_y = 0xFD;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
            assert_eq!(bus.cpu.index_y, 0xFE);
        }
    }

    #[test]
    fn jmp() {
        let test_code = vec![0x00, 0x10, 0xFF];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::JMP(AddressMode::Absolute);
        execute(&mut bus, &instruction);
        assert_eq!(bus.cpu.program_counter, 0x1000);
    }

    #[test]
    fn jsr() {
        let test_code = vec![0x03, 0x80, 0xFF, 0x02, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::JSR(AddressMode::Absolute);
        execute(&mut bus, &instruction);
        execute(&mut bus, &instruction);
        assert_eq!(bus.cpu.program_counter, 0x8002);
        assert_eq!(bus.cpu.stack_ptr, Cpu::STACK_PTR_INIT - 4);
    }

    #[test]
    fn lda() {
        let test_code = vec![0x00, 0xFF, 0x01];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::LDA(AddressMode::Immediate);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x01);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn ldx() {
        let test_code = vec![0x00, 0xFF, 0x01];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::LDX(AddressMode::Immediate);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0x01);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn ldy() {
        let test_code = vec![0x00, 0xFF, 0x01];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::LDY(AddressMode::Immediate);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_y, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_y, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_y, 0x01);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn lsr() {
        let test_code = vec![0x02, 0x80, 0x08];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::LSR(AddressMode::Accumulator);
        let base_status = Status::default();
        {
            bus.cpu.accumulator = 0x80;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80 >> 1);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.accumulator = 0x2;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x1);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Carry | Status::Zero);
        }
        {
            let instruction = Instruction::LSR(AddressMode::Absolute);
            execute(&mut bus, &instruction);
            assert_eq!(bus.cart.mem[2], 4);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn nop() {}

    #[test]
    fn ora() {
        let test_code = vec![0x00, 0x01, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::ORA(AddressMode::Immediate);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x1);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x81);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
    }

    #[test]
    fn pha() {
        let test_code = vec![0x00, 0x01, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::PHA(AddressMode::Implied);
        {
            bus.cpu.accumulator = 0x10;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, bus.ram.memory[0x1FF]);
            assert_eq!(bus.cpu.stack_ptr, 0xFE);
        }
        {
            bus.cpu.accumulator = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, bus.ram.memory[0x1FE]);
            assert_eq!(bus.cpu.stack_ptr, 0xFD);
        }
    }

    #[test]
    fn pla() {
        let test_code = vec![0x00, 0x01, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::PLA(AddressMode::Implied);
        {
            let instruction = Instruction::PHA(AddressMode::Implied);
            bus.cpu.accumulator = 0x10;
            execute(&mut bus, &instruction);
            bus.cpu.accumulator = 0x20;
            execute(&mut bus, &instruction);
            bus.cpu.accumulator = 0;
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x20);
            assert_eq!(bus.cpu.stack_ptr, 0xFE);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x10);
            assert_eq!(bus.cpu.stack_ptr, 0xFF);
        }
    }

    #[test]
    fn php() {
        let test_code = vec![0x00, 0x01, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::PHP(AddressMode::Implied);
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, bus.ram.memory[0x1FF]);
            assert_eq!(bus.cpu.stack_ptr, 0xFE);
        }
        {
            bus.cpu.status = Status::all_bits();
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, bus.ram.memory[0x1FE]);
            assert_eq!(bus.cpu.stack_ptr, 0xFD);
        }
    }

    #[test]
    fn plp() {
        let test_code = vec![0x00, 0x01, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::PLP(AddressMode::Implied);
        {
            let instruction = Instruction::PHP(AddressMode::Implied);
            execute(&mut bus, &instruction);
            bus.cpu.status = Status::all_flags();
            execute(&mut bus, &instruction);
            bus.cpu.status = Status::default();
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, Status::all_flags());
            assert_eq!(bus.cpu.stack_ptr, 0xFE);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, Status::default());
            assert_eq!(bus.cpu.stack_ptr, 0xFF);
        }
    }

    #[test]
    fn rol() {
        let test_code = vec![0x02, 0x80, 0x02];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::ROL(AddressMode::Accumulator);
        let base_status = Status::default();
        bus.cpu.accumulator = 0x80 >> 2;
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80 >> 1);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Carry | Status::Zero);
        }
        {
            let instruction = Instruction::ROL(AddressMode::Absolute);
            execute(&mut bus, &instruction);
            assert_eq!(bus.cart.mem[2], 5);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn ror() {
        let test_code = vec![0x02, 0x80, 0x02];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::ROR(AddressMode::Accumulator);
        let base_status = Status::default();
        bus.cpu.accumulator = 0x01;
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(bus.cpu.status, base_status | Status::Carry | Status::Zero);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x80);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
        {
            let instruction = Instruction::ROR(AddressMode::Absolute);
            execute(&mut bus, &instruction);
            assert_eq!(bus.cart.mem[2], 1);
            assert_eq!(bus.cpu.status, base_status);
        }
    }

    #[test]
    fn rti() {}

    #[test]
    fn rts() {
        let test_code = vec![0x06, 0x80, 0xFF, 0, 0, 0, 0x02, 0x80];
        let mut bus = Bus::new(TestCart::new(test_code));
        {
            let instruction = Instruction::JSR(AddressMode::Absolute);
            execute(&mut bus, &instruction);
            execute(&mut bus, &instruction);
        }
        {
            let instruction = Instruction::RTS(AddressMode::Implied);
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.program_counter, 0x8008);
            assert_eq!(bus.cpu.stack_ptr, 0xFD);
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.program_counter, 0x8002);
            assert_eq!(bus.cpu.stack_ptr, 0xFF);
        }
    }

    #[test]
    fn sbc() {
        let test_code = vec![0x01, 0x01, 0x7D, 0x01];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::SBC(AddressMode::Immediate);
        let base_status = Status::default();
        bus.cpu.accumulator = 0x80;
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x7E);
            assert_eq!(bus.cpu.status, base_status | Status::Overflow | Status::Carry);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x7D);
            assert_eq!(bus.cpu.status, base_status | Status::Carry);
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0);
            assert_eq!(
                bus.cpu.status,
                base_status | Status::Zero | Status::Carry
            );
        }
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
    }

    #[test]
    fn sec() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::SEC(AddressMode::Implied);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Carry);
        }
    }

    #[test]
    fn sed() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::SED(AddressMode::Implied);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::Decimal);
        }
    }

    #[test]
    fn sei() {
        let test_code = vec![0x01, 0x7F, (-2i8) as u8];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::SEI(AddressMode::Implied);
        let base_status = Status::default();
        {
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.status, base_status | Status::InturruptDisable);
        }
    }

    #[test]
    fn sta() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::STA(AddressMode::Absolute);
        {
            bus.cpu.accumulator = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[0x01], 0x20);
        }
        {
            bus.cpu.accumulator = 0x80;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[0x05], 0x80);
        }
    }

    #[test]
    fn stx() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::STX(AddressMode::Absolute);
        {
            bus.cpu.index_x = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[0x01], 0x20);
        }
        {
            bus.cpu.index_x = 0x80;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[0x05], 0x80);
        }
    }

    #[test]
    fn sty() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::STY(AddressMode::Absolute);
        {
            bus.cpu.index_y = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[0x01], 0x20);
        }
        {
            bus.cpu.index_y = 0x80;
            execute(&mut bus, &instruction);
            assert_eq!(bus.ram.memory[0x05], 0x80);
        }
    }

    #[test]
    fn tax() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::TAX(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.accumulator = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0x20);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.accumulator = 0x0;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0x0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            bus.cpu.accumulator = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
    }

    #[test]
    fn tay() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::TAY(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.accumulator = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_y, 0x20);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.accumulator = 0x0;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_y, 0x0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            bus.cpu.accumulator = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_y, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
    }

    #[test]
    fn tsx() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::TSX(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.stack_ptr = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0x20);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.stack_ptr = 0x0;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0x0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            bus.cpu.stack_ptr = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.index_x, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
    }


    #[test]
    fn txa() {
        let test_code = vec![0x01, 0x00, 0x05, 0x00];
        let mut bus = Bus::new(TestCart::new(test_code));
        let instruction = Instruction::TXA(AddressMode::Implied);
        let base_status = Status::default();
        {
            bus.cpu.index_x = 0x20;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x20);
            assert_eq!(bus.cpu.status, base_status);
        }
        {
            bus.cpu.index_x = 0x0;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0x0);
            assert_eq!(bus.cpu.status, base_status | Status::Zero);
        }
        {
            bus.cpu.index_x = 0xFF;
            execute(&mut bus, &instruction);
            assert_eq!(bus.cpu.accumulator, 0xFF);
            assert_eq!(bus.cpu.status, base_status | Status::Negative);
        }
    }
}
