mod mandelbrot;
pub use mandelbrot::Mandelbrot;
use rug::Complex;

use crate::app::App;

pub trait Fractal {
    fn get(p: Complex, app: &App) -> i32;
}
