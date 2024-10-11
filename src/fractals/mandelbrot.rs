use rug::ops::{CompleteRound, Pow};
use rug::Complex;

use crate::app::App;
use crate::fractals::Fractal;
pub struct Mandelbrot;

impl Fractal for Mandelbrot {
    /// Implement the formula for the mandelbrot set,
    /// takes a complex number which corresponds to a point in the canvas,
    /// and see if it diverges. It if does, return the number of iterations
    /// and if does not, return -1
    fn get(p: Complex, app: &App) -> i32 {
        // iteration counter
        let mut n: i32 = 0;
        // Current term of the series
        let mut z: Complex = Complex::new(app.render_settings.prec);

        // Compute the next term while z is not beyond 2
        // from the origin and the maximum divergence is not passed
        while *z
            .abs_ref()
            .complete((app.render_settings.prec, app.render_settings.prec))
            .real()
            < 2
            && n < app.render_settings.max_diverg
        {
            z = z.pow(2) + &p;
            n += 1;
        }

        if n == app.render_settings.max_diverg {
            return -1;
        }

        n
    }
}
