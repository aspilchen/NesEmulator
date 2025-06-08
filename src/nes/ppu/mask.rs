use bitmask_enum::bitmask;

#[bitmask(u8)]
#[derive(Default)]
pub enum Mask {
    Greyscale,
    LeftmostBackground,
    LeftmostSprite,
    RenderBackground,
    RenderSprites,
    EmphasizeRed,
    EmphasizeGreen,
    EmphasizeBlue,
}
