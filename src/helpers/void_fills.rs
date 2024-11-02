use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, Display)]
pub(crate) enum VoidFill {
    Transparent,
    Black,
    White,
    GreenNoise,
    BlueNoise,
    RedNoise,
    RGBNoise,
    ColorScheme,
}

// Todo: find a way to make this a constant
pub(crate) fn void_fills() -> Vec<VoidFill> {
    VoidFill::iter().collect()
}

