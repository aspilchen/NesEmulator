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

use crate::memory::Memory;

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

    pub fn new(chr_rom: Vec<u8>, mirroring: VramMirror) -> Self {
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
            chr_rom: chr_rom,
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

    pub fn ram_read(&mut self, address: usize) -> u8 {
        let result = self.buffer;
        self.buffer = self.vram.read(address);
        return result;
    }

    pub fn ram_write(&mut self, address: usize, value: u8) {
        let result = self.vram.write(address, value);
    }

    pub fn chr_read(&mut self, address: usize) -> u8 {
        let result = self.buffer;
        self.buffer = self.chr_rom[address];
        return result;
    }

    pub fn chr_write(&mut self, address: usize, value: u8) {
        self.chr_rom[address] = value;
    }

    pub fn scroll_write(&mut self, value: u8) {
        todo!()
    }

    fn map_address(&self, address: usize) -> usize {
        let mask = 0x7;
        return address & mask;
    }

    fn increment_address(&mut self) {
        let increment = self.control.get_increment();
        self.address.increment(increment);
    }
}
