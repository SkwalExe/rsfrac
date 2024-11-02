//! Contains the `RenderSettings` struct.

use rug::{Complex, Float};

use crate::frac_logic::CanvasCoords;
use crate::fractals::FRACTALS;

const DEFAULT_PREC: u32 = 32;
const DEFAULT_MAX_ITER: i32 = 32;

/// Used to group values related to fractal rendering logic.
pub(crate) struct RenderSettings {
    /// The size of one canvas cell.
    pub(crate) cell_size: Float,
    /// The position of the middle of the canvas in the complex plane.
    pub(crate) pos: Complex,
    /// The size of the canvas in Canvas Coordinates.
    pub(crate) canvas_size: CanvasCoords,
    /// The decimal precision (bit-length) used for calculations.
    pub(crate) prec: u32,
    /// The maximum number of iterations before assuming that a point diverges.
    pub(crate) max_iter: i32,
    /// The index of the currently selected fractal.
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
