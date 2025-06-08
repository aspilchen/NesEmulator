use bitmask_enum::bitmask;

#[bitmask(u8)]
pub enum Status {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    B,
    Unused,
    Overflow,
    Negative,
}

impl Default for Status {
    fn default() -> Self {
        Status::Unused | Status::InterruptDisable
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
