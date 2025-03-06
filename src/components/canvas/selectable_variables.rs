use strum::{Display, EnumIter, IntoEnumIterator};

/// Represents all the values that can be selected and controlled from the canvas component
#[derive(PartialEq, EnumIter, Debug, Display, Clone)]
pub(crate) enum SelectedVariable {
    PaletteOffset,
    HSLSmoothness,
    HueOffset,
    HSLSat,
    HSLLum,
}

impl PartialEq<usize> for SelectedVariable {
    fn eq(&self, other: &usize) -> bool {
        // Return true if the position of self in selectable_variables() is other
        selectable_variables()
            .iter()
            .position(|x| x == self)
            .unwrap()
            == *other // The variant MUST have a position in selectable_variables()
    }
}

pub(crate) fn selectable_variables() -> Vec<SelectedVariable> {
    SelectedVariable::iter().collect()
}
