use std::ops::Deref;

use rug::{Complex, Float};

use crate::helpers::Vec2;

use super::render_settings::RenderSettings;

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

#[derive(Clone, Debug, Default)]
pub(crate) struct CanvasCoords(Vec2<i32>);

impl CanvasCoords {
    pub(crate) fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        Self(Vec2::new(x, y))
    }
}

// Implement Deref to delegate method calls to Vec2<i32>
impl Deref for CanvasCoords {
    type Target = Vec2<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderSettings {
    pub(crate) fn coord_to_c_with_cell_size(
        &self,
        coords: CanvasCoords,
        cell_size: &Float,
    ) -> Complex {
        Complex::with_val(self.prec, (coords.0.x * cell_size, coords.0.y * cell_size)) + &self.pos
    }

    /// Takes coordinates on the canvas and return the
    /// complex number at the corresponding position on the complex plane
    pub(crate) fn coord_to_c(&self, coords: CanvasCoords) -> Complex {
        self.coord_to_c_with_cell_size(coords, &self.cell_size)
    }
}
