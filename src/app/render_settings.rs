use rug::{ops::CompleteRound, Complex, Float};

use crate::fractals::{Fractal, FractalClos, FRACTALS};

use super::fractal_logic::CanvasCoords;

pub(crate) const DEFAULT_PREC: u32 = 32;
pub(crate) const DEFAULT_MAX_ITER: i32 = 32;

pub(crate) struct RenderSettings {
    pub(crate) cell_size: Float,
    pub(crate) pos: Complex,
    // The size of the canvas in Canvas Coordinates
    pub(crate) canvas_size: CanvasCoords,
    pub(crate) prec: u32,
    pub(crate) max_iter: i32,
    pub(crate) frac_index: usize,
}
impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            frac_index: Default::default(),
            pos: Complex::with_val(DEFAULT_PREC, FRACTALS[0].default_pos),
            max_iter: DEFAULT_MAX_ITER,
            cell_size: Float::new(DEFAULT_PREC),
            canvas_size: Default::default(),
            prec: DEFAULT_PREC,
        }
    }
}

impl RenderSettings {
    /// Return the total number of points to render in the canvas
    pub(crate) fn point_count(&self) -> i32 {
        self.canvas_size.x * self.canvas_size.y
    }

    /// Set the position of the canvas to the default position
    pub(crate) fn reset_pos(&mut self) {
        self.pos = self.get_default_pos();
    }

    /// Return a complex corresponding to the default position,
    /// with the decimal precision configured on the App.
    pub(crate) fn get_default_pos(&self) -> Complex {
        Complex::with_val(self.prec, self.get_frac_obj().default_pos)
    }
    pub(crate) fn get_zoom(&self) -> Float {
        self.get_default_cell_size() / &self.cell_size
    }
    pub(crate) fn get_frac_index_by_name(&self, name: &str) -> Option<usize> {
        FRACTALS
            .iter()
            .position(|f| f.name.to_lowercase() == name.to_lowercase())
    }
    pub(crate) fn get_frac_clos(&self) -> FractalClos {
        FRACTALS[self.frac_index].get
    }
    pub(crate) fn get_frac_obj(&self) -> &Fractal {
        &FRACTALS[self.frac_index]
    }
    pub(crate) fn ratatui_to_canvas_coords(&self, x: u16, y: u16) -> CanvasCoords {
        // I don't understand how this works
        CanvasCoords::new(
            x as i32 - 1 - self.canvas_size.x / 2,
            y as i32 * -2 + 1 + self.canvas_size.y / 2,
        )
    }
    /// Set the cell size so that the total width of the canvas is 4 on the real axis
    pub(crate) fn reset_cell_size(&mut self) {
        self.cell_size = self.get_default_cell_size();
    }

    /// Return the cell size so that the total width of the canvas is 4 on the real axis
    pub(crate) fn get_default_cell_size(&self) -> Float {
        Float::with_val(self.prec, 4) / self.canvas_size.x
    }
    pub(crate) fn get_plane_wid(&self) -> Float {
        (self.canvas_size.x * &self.cell_size).complete(self.prec)
    }
}
