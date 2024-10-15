use rug::ops::{CompleteRound, Pow};
use rug::Complex;

use crate::app::RenderSettings;
use crate::fractals::Fractal;
pub struct Mandelbrot;

impl Fractal for Mandelbrot {
    /// Implement the formula for the mandelbrot set,
    /// takes a complex number which corresponds to a point in the canvas,
    /// and see if it diverges. It if does, return the number of iterations
    /// and if does not, return -1
    fn get(p: Complex, render_settings: &RenderSettings) -> i32 {
        // iteration counter
        let mut n: i32 = 0;
        // Current term of the series
        let mut z: Complex = Complex::new(render_settings.prec);

        // Compute the next term while z is not beyond 2
        // from the origin and the maximum divergence is not passed
        while *z
            .abs_ref()
            .complete((render_settings.prec, render_settings.prec))
            .real()
            < 2
            && n < render_settings.max_iter
        {
            z = z.pow(2) + &p;
            n += 1;
        }

        if n == render_settings.max_iter {
            return -1;
        }

        n
    }
}
