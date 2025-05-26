use bitmask_enum::bitmask;

#[bitmask(u8)]
#[bitmask_config(inverted_flags)]
#[derive(Default)]
pub enum Control {
    BaseNametableAddress1,
    BaseNametableAddress2,
    VramAddressIncrement,
    SpritePatternAddress,
    BackgroundPatternAddress,
    SpriteSize,
    MasterSlaveSelect,
    NmiEnable,
}

impl Control {
    pub fn get_nametable_base(&self) -> usize {
        let mut result = 0x2000;
        let bit_1: bool = self.contains(Control::BaseNametableAddress1);
        let bit_2: bool = self.contains(Control::BaseNametableAddress2);
        if (bit_1) {
            result += 0x400;
        }
        if (bit_2) {
            result += 0x800;
        }
        return result;
    }

    pub fn get_sprite_pattern_address(&self) -> usize {
        let result = if self.contains(Control::SpritePatternAddress) {
            0x1000
        } else {
            0
        };
        return result;
    }

    pub fn get_background_pattern_address(&self) -> usize {
        let result = if self.contains(Control::BackgroundPatternAddress) {
            0x1000
        } else {
            0
        };
        return result;
    }

    pub fn get_increment(&self) -> u16 {
        return if (self.contains(Control::VramAddressIncrement)) {
            32
        } else {
            1
        };
    }
}
