//! Contains the `Canvas` widget.

use crate::{app::CanvasPoints, AppState};

pub(crate) struct Canvas<'a> {
    pub(crate) state: &'a AppState,
    pub(crate) points: &'a CanvasPoints,
}

impl<'a> Canvas<'a> {
    pub(crate) const FOOTER_TEXT: &'static [&'static str] = &[
        "Move[arrow/Vim keys]",
        "Zoom-[s]",
        "Zoom+[d]",
        "NextVar[t]",
        "SelectedVar+[+]",
        "Selectedvar-[-]",
        "MxDiv-[y]",
        "Prec-[u]",
        "Prec+[i]",
        "MxDiv+[o]",
        "Color[c]",
        "Frac[f]",
        "VoidFill[v]",
        "Rst[r]",
        "HSLMode[n]",
        "ToggleSidepanel[b]",
    ];
    pub(crate) fn new(state: &'a AppState, points: &'a CanvasPoints) -> Self {
        Self { state, points }
    }
}
