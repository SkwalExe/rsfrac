use strum::Display;

#[derive(Debug, Display, Clone)]
pub(crate) enum ClickMode {
    ZoomIn,
    ZoomOut,
    JuliaConstant,
    Move,
}

impl ClickMode {
    pub(crate) fn all() -> &'static [ClickMode] {
        &[
            ClickMode::ZoomIn,
            ClickMode::ZoomOut,
            ClickMode::Move,
            ClickMode::JuliaConstant,
        ]
    }

    pub fn from(str_: &str) -> Option<Self> {
        Some(match str_.to_lowercase().as_str() {
            "zoomin" => Self::ZoomIn,
            "zoomout" => Self::ZoomOut,
            "move" => Self::Move,
            "juliaconstant" => Self::JuliaConstant,
            _ => return None,
        })
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
