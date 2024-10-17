use rug::ops::{CompleteRound, Pow};
use rug::Complex;

use crate::app::RenderSettings;
use crate::fractals::Fractal;

/// Implement the formula for the mandelbrot set,
/// takes a complex number which corresponds to a point in the canvas,
/// and see if it diverges. It if does, return the number of iterations
/// and if does not, return -1
fn get_mandelbrot(p: Complex, render_settings: &RenderSettings) -> i32 {
    // iteration counter
    let mut n: i32 = 0;
    // Current term of the series

    // Todo: allow to change this
    let mandelbrot_u0 = (0, 0);
    let mut z: Complex = Complex::with_val(render_settings.prec, mandelbrot_u0);

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

pub const MANDELBROT: Fractal = Fractal {
    get: &get_mandelbrot,
    name: "Mandelbrot",
    details: concat!(
        "Default Formula: \n<acc Uₙ₊₁ = Uₙ²+P >\n",
        // Todo: allow custom exp
        "General case: \n<acc Uₙ₊₁ = Uₙ²+P >\n",
        "Where <acc P> is the complex number at the position of the pixel, ",
        "and <acc U₀> is a constant that can be modified, set to <acc 0+0j> by default.\n",
    ),
    default_pos: (-0.5, 0.0)
};

