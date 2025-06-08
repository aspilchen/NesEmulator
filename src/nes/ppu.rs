mod address;
mod control;
mod mask;
mod status;
mod vram;

use address::AddressRegister;
use control::Control;
use mask::Mask;
use status::Status;
use vram::{VRam, VramMirror};

use crate::nes::cart::Cart;

use super::memory::Memory;

pub const CPU_TO_PPU_CYCLE_SCALE: i64 = 3;
const NUM_SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: i64 = 341;
const SCANLINE_NMI_TRIGGER: usize = 241;

pub struct Ppu {
    pub control: Control,
    pub mask: Mask,
    pub status: Status,
    pub oam_address: u8,
    pub scroll: u8,
    pub oam_dma: u8,
    pub oam_data: [u8; Self::OAM_DATA_SIZE],
    pub address: AddressRegister,
    pub vram: VRam,
    pub buffer: u8,
    pub chr_rom: Vec<u8>,
    pub clock: i64,
    pub curr_scanline: usize,
    pub is_nmi_interrupt: bool,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            control: Default::default(),
            mask: Default::default(),
            status: Default::default(),
            oam_address: Default::default(),
            scroll: Default::default(),
            oam_dma: Default::default(),
            vram: Default::default(),
            oam_data: [0; Self::OAM_DATA_SIZE],
            address: Default::default(),
            buffer: 0,
            chr_rom: vec![],
            clock: 0,
            curr_scanline: 0,
            is_nmi_interrupt: false,
        }
    }
}

impl Memory for Ppu {
    fn read(&mut self, address: usize) -> u8 {
        let address = self.map_address(address);
        let result = match address {
            Self::STATUS_ADDR => self.status.bits(),
            Self::OAM_DATA_ADDR => self.oam_read(),
            Self::DATA_ADDR => self.internal_read(),
            _ => panic!("invalid read at {}", address),
        };
        return result;
    }

    fn write(&mut self, address: usize, value: u8) {
        let address = self.map_address(address);
        match address {
            Self::CONTROL_ADDR => self.control = Control::from(value),
            Self::MASK_ADDR => self.mask = Mask::from(value),
            Self::OAM_ADDR_ADDR => self.set_oam_address(value),
            Self::OAM_DATA_ADDR => self.oam_write(value),
            Self::SCROLL_ADDR => self.scroll_write(value),
            Self::ADDR_ADDR => self.address.write(value),
            Self::DATA_ADDR => self.internal_write(value),
            Self::OAM_DMA_ADDR => todo!(),
            _ => panic!("invalid write to 0x{:04X}", address),
        }
    }
}

impl Ppu {
    pub const BEGIN: usize = 0x2000;
    pub const END: usize = 0x3FFF;
    pub const DMA: usize = 0x4017;
    pub const OAM_DATA_SIZE: usize = 0x100;

    pub const CONTROL_ADDR: usize = 0x2000;
    pub const MASK_ADDR: usize = 0x2001;
    pub const STATUS_ADDR: usize = 0x2002;
    pub const OAM_ADDR_ADDR: usize = 0x2003;
    pub const OAM_DATA_ADDR: usize = 0x2004;
    pub const SCROLL_ADDR: usize = 0x2005;
    pub const ADDR_ADDR: usize = 0x2006;
    pub const DATA_ADDR: usize = 0x2007;
    pub const OAM_DMA_ADDR: usize = 0x4014;

    pub fn new(cart: &Cart) -> Self {
        return Self {
            control: Default::default(),
            mask: Default::default(),
            status: Default::default(),
            oam_address: Default::default(),
            scroll: Default::default(),
            oam_dma: Default::default(),
            oam_data: [0; Self::OAM_DATA_SIZE],
            address: Default::default(),
            vram: Default::default(),
            buffer: Default::default(),
            chr_rom: cart.chr_rom.to_vec(),
            clock: 0,
            curr_scanline: 0,
            is_nmi_interrupt: false,
        };
    }

    pub fn oam_read(&mut self) -> u8 {
        let address = self.oam_address as usize;
        return self.oam_data[address];
    }

    pub fn oam_write(&mut self, value: u8) {
        let address = self.oam_address as usize;
        self.oam_address = self.oam_address.wrapping_add(1);
        self.oam_data[address] = value;
    }

    pub fn set_oam_address(&mut self, value: u8) {
        self.oam_address = value;
    }

    pub fn tick(&mut self, n_cycles: i64) {
        self.clock += n_cycles;
        if (self.clock >= CYCLES_PER_SCANLINE) {
            self.clock -= CYCLES_PER_SCANLINE;
            self.curr_scanline += 1;
            if (self.curr_scanline == SCANLINE_NMI_TRIGGER
                && self.control.contains(Control::NmiEnable))
            {
                self.is_nmi_interrupt = true;
            }
            if (self.curr_scanline >= NUM_SCANLINES) {
                self.curr_scanline = 0;
                self.is_nmi_interrupt = false;
            }
        }
    }

    pub fn render_chr_tile(&mut self, bank: usize, tile_n: usize) -> Frame {
        let mut frame = Frame::new();
        let bank = (bank * 0x1000) as usize;

        let tile = &self.chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                frame.write(x, y, rgb)
            }
        }
        return frame;
    }

    pub fn render_chr_tile_bank(chr_rom: &Vec<u8>, bank: usize) -> Frame {
        assert!(bank <= 1);

        let mut frame = Frame::new();
        let mut tile_y = 0;
        let mut tile_x = 0;
        let bank = (bank * 0x1000) as usize;

        for tile_n in 0..255 {
            if tile_n != 0 && tile_n % 20 == 0 {
                tile_y += 10;
                tile_x = 0;
            }
            let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & upper) << 1 | (1 & lower);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => SYSTEM_PALLETE[0x01],
                        1 => SYSTEM_PALLETE[0x23],
                        2 => SYSTEM_PALLETE[0x27],
                        3 => SYSTEM_PALLETE[0x30],
                        _ => panic!("can't be"),
                    };
                    frame.write(tile_x + x, tile_y + y, rgb)
                }
            }

            tile_x += 10;
        }
        return frame;
    }

    fn internal_read(&mut self) -> u8 {
        let chr_rom = 0..=0x1FFF;
        let vram = 0x2000..=0x2FFF;
        let pallet_table = 0x3F00..=0x3FFF;
        let address = self.address.read() as usize;
        self.increment_address();
        let result = match address {
            chr_rom => self.chr_read(address),
            vram => self.ram_read(address),
            pallet_table => todo!(),
            _ => panic!("invalid ppu read at internal address 0x{:04X}", address),
        };
        return result;
    }

    fn internal_write(&mut self, value: u8) {
        let chr_rom = 0..=0x1FFF;
        let vram = 0x2000..=0x2FFF;
        let pallet_table = 0x3F00..=0x3FFF;
        let address = self.address.read() as usize;
        self.increment_address();
        let result = match address {
            chr_rom => self.chr_write(address, value),
            vram => self.ram_write(address, value),
            pallet_table => todo!(),
            _ => panic!("invalid ppu read at internal address 0x{:04X}", address),
        };
    }

    fn ram_read(&mut self, address: usize) -> u8 {
        let result = self.buffer;
        self.buffer = self.vram.read(address);
        return result;
    }

    fn ram_write(&mut self, address: usize, value: u8) {
        let result = self.vram.write(address, value);
    }

    fn chr_read(&mut self, address: usize) -> u8 {
        let result = self.buffer;
        self.buffer = self.chr_rom[address];
        return result;
    }

    fn chr_write(&mut self, address: usize, value: u8) {
        self.chr_rom[address] = value;
    }

    fn scroll_write(&mut self, value: u8) {
        todo!()
    }

    fn control_write(&mut self, value: u8) {
        let current_control = self.control;
        let new_control = Control::from(value);
        self.control = new_control;
        if (!current_control.contains(Control::NmiEnable)
            && new_control.contains(Control::NmiEnable)
            && self.status.contains(Status::VBlank))
        {
            self.is_nmi_interrupt = true;
        }
    }

    fn map_address(&self, address: usize) -> usize {
        // println!("{:X}",address);
        let mask = 0xFF;
        return address;
        return address & mask;
    }

    fn increment_address(&mut self) {
        let increment = self.control.get_increment();
        self.address.increment(increment);
    }
}

pub static SYSTEM_PALLETE: [(u8, u8, u8); 64] = [
    (0x80, 0x80, 0x80),
    (0x00, 0x3D, 0xA6),
    (0x00, 0x12, 0xB0),
    (0x44, 0x00, 0x96),
    (0xA1, 0x00, 0x5E),
    (0xC7, 0x00, 0x28),
    (0xBA, 0x06, 0x00),
    (0x8C, 0x17, 0x00),
    (0x5C, 0x2F, 0x00),
    (0x10, 0x45, 0x00),
    (0x05, 0x4A, 0x00),
    (0x00, 0x47, 0x2E),
    (0x00, 0x41, 0x66),
    (0x00, 0x00, 0x00),
    (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05),
    (0xC7, 0xC7, 0xC7),
    (0x00, 0x77, 0xFF),
    (0x21, 0x55, 0xFF),
    (0x82, 0x37, 0xFA),
    (0xEB, 0x2F, 0xB5),
    (0xFF, 0x29, 0x50),
    (0xFF, 0x22, 0x00),
    (0xD6, 0x32, 0x00),
    (0xC4, 0x62, 0x00),
    (0x35, 0x80, 0x00),
    (0x05, 0x8F, 0x00),
    (0x00, 0x8A, 0x55),
    (0x00, 0x99, 0xCC),
    (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09),
    (0x09, 0x09, 0x09),
    (0xFF, 0xFF, 0xFF),
    (0x0F, 0xD7, 0xFF),
    (0x69, 0xA2, 0xFF),
    (0xD4, 0x80, 0xFF),
    (0xFF, 0x45, 0xF3),
    (0xFF, 0x61, 0x8B),
    (0xFF, 0x88, 0x33),
    (0xFF, 0x9C, 0x12),
    (0xFA, 0xBC, 0x20),
    (0x9F, 0xE3, 0x0E),
    (0x2B, 0xF0, 0x35),
    (0x0C, 0xF0, 0xA4),
    (0x05, 0xFB, 0xFF),
    (0x5E, 0x5E, 0x5E),
    (0x0D, 0x0D, 0x0D),
    (0x0D, 0x0D, 0x0D),
    (0xFF, 0xFF, 0xFF),
    (0xA6, 0xFC, 0xFF),
    (0xB3, 0xEC, 0xFF),
    (0xDA, 0xAB, 0xEB),
    (0xFF, 0xA8, 0xF9),
    (0xFF, 0xAB, 0xB3),
    (0xFF, 0xD2, 0xB0),
    (0xFF, 0xEF, 0xA6),
    (0xFF, 0xF7, 0x9C),
    (0xD7, 0xE8, 0x95),
    (0xA6, 0xED, 0xAF),
    (0xA2, 0xF2, 0xDA),
    (0x99, 0xFF, 0xFC),
    (0xDD, 0xDD, 0xDD),
    (0x11, 0x11, 0x11),
    (0x11, 0x11, 0x11),
];

pub struct Frame {
    pub data: Vec<u8>,
}

impl Frame {
    const WIDTH: usize = 256;
    const HEIGH: usize = 240;

    pub fn new() -> Self {
        return Self {
            data: vec![0; Self::WIDTH * Self::HEIGH * 3],
        };
    }

    pub fn write(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        let index = self.map_index(x, y);
        self.data[index] = color.0;
        self.data[index + 1] = color.1;
        self.data[index + 2] = color.2;
    }

    fn map_index(&self, x: usize, y: usize) -> usize {
        return (x * 3) + (y * 3 * Self::WIDTH);
    }
}

pub fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let mut tile_y = 0;
    let mut tile_x = 0;
    let bank = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                frame.write(tile_x + x, tile_y + y, rgb)
            }
        }

        tile_x += 10;
    }
    frame
}
