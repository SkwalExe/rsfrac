//! Contains the algorithms for each fractal

use rug::Complex;

mod burning_ship;
mod julia;
mod mandelbrot;

pub(crate) use burning_ship::BURNING_SHIP;
pub(crate) use julia::JULIA;
pub(crate) use mandelbrot::MANDELBROT;

pub(crate) type FractalClos = &'static dyn Fn(Complex, &RenderSettings) -> i32;

use crate::frac_logic::RenderSettings;

/// Represents a fractal type.
pub(crate) struct Fractal {
    /// A closure that takes in `RenderSettings`, a complex and
    /// returns the number of iterations before it diverged,
    /// and `-1` if it reached the maximum number of iterations.
    pub(crate) get: FractalClos,
    /// The fractal display name.
    pub(crate) name: &'static str,
    /// Some details to display about the fractal (formula, etc).
    /// https://www.unicodeit.net/
    pub(crate) details: &'static str,
    /// The default position of the canvas when first rendering the fractal.
    pub(crate) default_pos: (f64, f64),
}

/// Returns the index of a fractal which name matches, or `None`.
pub(crate) fn get_frac_index_by_name(name: &str) -> Option<usize> {
    FRACTALS
        .iter()
        .position(|f| f.name.to_lowercase().starts_with(&name.to_lowercase()))
}

pub(crate) const FRACTALS: &[Fractal] = &[MANDELBROT, BURNING_SHIP, JULIA];
