use strum::Display;

#[derive(Debug, Display, Clone)]
pub(crate) enum ClickMode {
    ZoomIn,
    ZoomOut,
    JuliaConstant,
    MandelConstant,
    Move,
    BailOut,
    Info,
}

impl ClickMode {
    pub(crate) fn all() -> &'static [ClickMode] {
        &[
            ClickMode::ZoomIn,
            ClickMode::ZoomOut,
            ClickMode::Move,
            ClickMode::JuliaConstant,
            ClickMode::BailOut,
            ClickMode::MandelConstant,
            ClickMode::Info,
        ]
    }

    pub fn from(str_: &str) -> Option<Self> {
        [
            ("zoomin", Self::ZoomIn),
            ("zoomout", Self::ZoomOut),
            ("move", Self::Move),
            ("juliaconstant", Self::JuliaConstant),
            ("mandelconstant", Self::MandelConstant),
            ("bailout", Self::BailOut),
            ("info", Self::Info),
        ]
        .iter()
        .find(|x| x.0.starts_with(&str_.to_lowercase().replace("_", "")))
        .map(|x| x.1.clone())
    }
}

pub(crate) struct ClickConfig {
    pub(crate) left: ClickMode,
    pub(crate) middle: ClickMode,
    pub(crate) right: ClickMode,
}

impl Default for ClickConfig {
    fn default() -> Self {
        Self {
            left: ClickMode::ZoomIn,
            middle: ClickMode::Move,
            right: ClickMode::ZoomOut,
        }
    }
}
