use bitmask_enum::bitmask;

#[bitmask(u8)]
#[derive(Default)]
pub enum Status {
    OpenBus = 0b00011111,
    SpriteOverflow,
    SpriteZeroHit,
    VBlank,
}
