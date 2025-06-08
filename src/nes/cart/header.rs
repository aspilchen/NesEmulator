use bitmask_enum::bitmask;

const FORMAT_TAG_SIZE: usize = 4;
const ROM_BANK_SIZE: usize = 0x4000;

const PRG_BANKS_ADDRESS: usize = 4;
const CHR_BANKS_ADDRESS: usize = 5;
const CONTROL_1: usize = 7;
const CONTROL_2: usize = 8;
const RAM_BANK_ADDRESS: usize = 9;

#[derive(Debug)]
pub struct Header {
    pub format_tag: [u8; FORMAT_TAG_SIZE],
    pub num_prg_banks: u8,
    pub num_chr_banks: u8,
    pub control: Control,
    pub num_ram_banks: u8,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            format_tag: Default::default(),
            num_prg_banks: Default::default(),
            num_chr_banks: Default::default(),
            control: Default::default(),
            num_ram_banks: Default::default(),
        }
    }
}

impl Header {
    pub fn new(ines_data: &Vec<u8>) -> Self {
        return Self {
            format_tag: [ines_data[0], ines_data[1], ines_data[2], ines_data[3]],
            num_prg_banks: ines_data[PRG_BANKS_ADDRESS],
            num_chr_banks: ines_data[CHR_BANKS_ADDRESS],
            control: Control::new(ines_data[CONTROL_1], ines_data[CONTROL_2]),
            num_ram_banks: ines_data[RAM_BANK_ADDRESS],
        };
    }

    pub fn is_valid_tag(&self) -> bool {
        let ines_tag = [0x4E, 0x45, 0x53, 0x1A];
        return self.format_tag == ines_tag;
    }
}

#[derive(Debug)]
pub struct Control {
    pub control_one: ControlOne,
    pub control_two: ControlTwo,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            control_one: ControlOne::none(),
            control_two: ControlTwo::none(),
        }
    }
}

impl Control {
    pub fn new(first_byte: u8, second_byte: u8) -> Self {
        return Self {
            control_one: ControlOne::from(first_byte),
            control_two: ControlTwo::from(second_byte),
        };
    }

    pub fn get_mapper(&self) -> u8 {
        let upper = self.control_two & ControlTwo::Mapper;
        let lower = self.control_one & ControlOne::Mapper;
        let result = (upper.bits()) & (lower.bits() >> 4);
        return result;
    }
}
#[bitmask(u8)]
pub enum ControlOne {
    Mapper = 0xF0,
    VramLayout = 0b00001000,
    Trainer = 0b00000100,
    Ram = 0b00000010,
    Mirroring = 0b00000001,
}

#[bitmask(u8)]
pub enum ControlTwo {
    Mapper = 0xF0,
    InesFormat = 0b00001100,
}
