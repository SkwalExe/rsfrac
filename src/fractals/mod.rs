mod julia;
mod mandelbrot;
pub(crate) use julia::JULIA;
pub(crate) use mandelbrot::MANDELBROT;
use rug::Complex;

use crate::app::RenderSettings;

pub(crate) type FractalClos = &'static dyn Fn(Complex, &RenderSettings) -> i32;
pub(crate) struct Fractal {
    pub(crate) get: FractalClos,
    pub(crate) name: &'static str,
    pub(crate) details: &'static str,
    pub(crate) default_pos: (f64, f64),
}

pub(crate) const FRACTALS: &[Fractal] = &[MANDELBROT, JULIA];
