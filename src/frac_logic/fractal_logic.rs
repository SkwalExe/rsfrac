use crate::frac_logic::{CanvasCoords, RenderSettings};
use crate::fractals::{Fractal, FractalClos, FRACTALS};

use rug::ops::CompleteRound;
use rug::{Complex, Float};

const INITIAL_CANVAS_WIDTH: i32 = 5;

impl RenderSettings {
    /// Use the provided cell size to find the complex number
    /// corresponding to the given canvas position.
    /// Uses the configured precision and canvas position.
    pub(crate) fn coord_to_c_with_cell_size(
        &self,
        coords: CanvasCoords,
        cell_size: &Float,
    ) -> Complex {
        Complex::with_val(self.prec, (coords.x * cell_size, coords.y * cell_size)) + &self.pos
    }

    /// Takes coordinates on the canvas and return the
    /// complex number at the corresponding position on the complex plane.
    pub(crate) fn coord_to_c(&self, coords: CanvasCoords) -> Complex {
        self.coord_to_c_with_cell_size(coords, &self.cell_size)
    }

    /// Set the position of the canvas to the selected fractal's fixed default position.
    pub(crate) fn reset_pos(&mut self) {
        self.pos = Complex::with_val(self.prec, self.get_frac_obj().default_pos);
    }

    /// Returns the selected fractal's closure.
    pub(crate) fn get_frac_clos(&self) -> FractalClos {
        FRACTALS[self.frac_index].get
    }

    /// Returns the selected fractal.
    pub(crate) fn get_frac_obj(&self) -> &Fractal {
        &FRACTALS[self.frac_index]
    }

    /// Converts ratatui coordinates to canvas coordinates.
    pub(crate) fn ratatui_to_canvas_coords(&self, x: u16, y: u16) -> CanvasCoords {
        // I don't understand how this works
        CanvasCoords::new(
            x as i32 - 1 - self.canvas_size.x / 2,
            y as i32 * -2 + 1 + self.canvas_size.y / 2,
        )
    }

    /// Set the cell size so that the total width of the canvas is 4 on the real axis.
    pub(crate) fn reset_cell_size(&mut self) {
        self.cell_size = self.get_default_cell_size();
    }

    /// Return the cell size so that the total width of the canvas is 4 on the real axis.
    pub(crate) fn get_default_cell_size(&self) -> Float {
        Float::with_val(self.prec, INITIAL_CANVAS_WIDTH) / self.canvas_size.x
    }

    /// Get the canvas width in the complex plane.
    pub(crate) fn get_plane_wid(&self) -> Float {
        (self.canvas_size.x * &self.cell_size).complete(self.prec)
    }
}
