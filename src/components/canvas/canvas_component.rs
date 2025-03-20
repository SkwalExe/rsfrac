//! Contains the `Canvas` widget.

use crate::{app::CanvasPoints, AppState};

pub(crate) struct Canvas<'a> {
    pub(crate) state: &'a AppState,
    pub(crate) points: &'a CanvasPoints,
}

impl<'a> Canvas<'a> {
    pub(crate) const FOOTER_TEXT: &'static [&'static str] = &[
        "Move[arrows/hjkl]",
        "+/-Zoom[s/d]",
        "NextVar[t]",
        "+/-Var[+/-]",
        "+/-MxDiv[y/o]",
        "+/-Prec[u/i]",
        "Color[c]",
        "Frac[f]",
        "Void[v]",
        "Rst[r]",
        "HSL[n]",
        "Panel[b]",
    ];
    pub(crate) fn new(state: &'a AppState, points: &'a CanvasPoints) -> Self {
        Self { state, points }
    }
}
