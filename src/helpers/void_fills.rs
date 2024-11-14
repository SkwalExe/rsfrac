use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(PartialEq, EnumIter, Debug, Display, Clone, Deserialize, Serialize)]
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
