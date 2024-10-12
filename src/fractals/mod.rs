mod mandelbrot;
pub use mandelbrot::Mandelbrot;
use rug::Complex;

use crate::app::RenderSettings;

pub trait Fractal {
    fn get(p: Complex, app: &RenderSettings) -> i32;
}
