use bitmask_enum::bitmask;

#[bitmask(u8)]
#[bitmask_config(inverted_flags)]
#[derive(Default)]
pub enum Mask {
    Greyscale,
    LeftmostBackground,
    LeftmostSprite,
    RenderBackground,
    RenderSprites,
    EmphasizeRed,
    EmphasizeGreen,
    EmphasizeB,
}
