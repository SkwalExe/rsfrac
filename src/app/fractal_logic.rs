use std::ops::Deref;

use rug::Complex;

use crate::helpers::{Vec2, ZoomDirection};

use super::App;

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

pub fn ratatui_to_canvas_coords(app: &App, x: u16, y: u16) -> CanvasCoords {
    // I don't understand how this works
    CanvasCoords::new(
        x as i32 - app.chunks.canvas_inner.x as i32 - app.render_settings.canvas_size.x / 2,
        y as i32 * -2
            + app.chunks.canvas_inner.y as i32
            + app.render_settings.canvas_size.y / 2,
    )
}

#[derive(Clone, Debug, Default)]
pub struct CanvasCoords(Vec2<i32>);

impl CanvasCoords {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
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

impl App {
    pub fn zoom_at(&mut self, pos: CanvasCoords, direction: ZoomDirection) {
        let inintial_c_pos = self.coord_to_c(pos.clone());
        self.zoom(direction);
        let new_c_pos = self.coord_to_c(pos);

        self.render_settings.pos += inintial_c_pos - new_c_pos;
    }
    /// Takes coordinates on the canvas and return the
    /// complex number at the corresponding position on the complex plane
    pub fn coord_to_c(&self, coords: CanvasCoords) -> Complex {
        Complex::with_val(
            self.render_settings.prec,
            (
                coords.0.x * &self.render_settings.cell_size,
                coords.0.y * &self.render_settings.cell_size,
            ),
        ) + &self.render_settings.pos
    }

    pub fn zoom(&mut self, direction: ZoomDirection) {
        // todo: Make configurable at app level
        let scaling_factor = 1.2;

        match direction {
            ZoomDirection::In => self.render_settings.cell_size /= scaling_factor,
            ZoomDirection::Out => self.render_settings.cell_size *= scaling_factor,
        }
    }
}
