//! Contains the CanvasCoords struct

// Coordinate system ============
// Ratatui Coordinates:
// One cell = (1;1)
// ___________________
// |0;0→          w;0|
// |↓                |
// |                 |
// |                 |
// |h;0           w;h|
// -------------------
//
// Canvas Coordinates:
// One point = (1;2)
// If the terminal height or width is even,
// the middle is always the one above/on the right
// _______________________
// |-w/2;h/2      w/2;h/2|
// |           ^         |
// |         (0;0)→      |
// |                     |
// |-w/2;-h/2    w/2;-h/2|
// ----------------------
//
// Complex Coordinates:
// _______________________
// |          0+1j       |
// |           ^         |
// |-2+0j    0+0j→   2+0j|
// |                     |
// |          0-1j       |
// ----------------------
//

use std::ops::Deref;

use crate::helpers::Vec2;

#[derive(Clone, Debug, Default)]
pub(crate) struct CanvasCoords(Vec2<i32>);

impl CanvasCoords {
    /// Creates a `CanvasCoords` using the provided `x` and `y` values,
    /// casting them if needed.
    pub(crate) fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        Self(Vec2::new(x.into(), y.into()))
    }
}

// Implement Deref to delegate method calls and properties to `Vec2<i32>`.
impl Deref for CanvasCoords {
    type Target = Vec2<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
