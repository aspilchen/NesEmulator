use bitmask_enum::bitmask;

#[bitmask(u8)]
#[bitmask_config(inverted_flags)]
pub enum Status {
    Carry,
    Zero,
    InturruptDisable,
    Decimal,
    B,
    Unused,
    Overflow,
    Negative,
}

impl Default for Status {
    fn default() -> Self {
        Status::Unused | Status::InturruptDisable
    }
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
