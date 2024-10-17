mod mandelbrot;
mod julia;
pub use mandelbrot::MANDELBROT;
pub use julia::JULIA;
use rug::Complex;

use crate::app::RenderSettings;

pub type FractalClos = &'static dyn Fn(Complex, &RenderSettings) -> i32;
pub struct Fractal {
    pub get: FractalClos,
    pub name: &'static str,
    pub details: &'static str,
    pub default_pos: (f64, f64)
}

pub const FRACTALS: &[Fractal] = &[
    MANDELBROT,
    JULIA
];
