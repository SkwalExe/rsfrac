//! Contains the Julia set rendering information.

use std::ops::{AddAssign, MulAssign};

use rug::ops::{CompleteRound, PowAssign};
use rug::Complex;

use crate::frac_logic::RenderSettings;
use crate::fractals::Fractal;

/// Implement the formula for the julia set,
fn get_burning_ship(mut p: Complex, render_settings: &RenderSettings) -> i32 {
    // invert the x and y axis
    p.mul_assign(-1);

    // iteration counter
    let mut n: i32 = 0;

    let mut z = Complex::new(render_settings.prec);
    while *z
        .abs_ref()
        .complete((render_settings.prec, render_settings.prec))
        .real()
        < 4
        && n < render_settings.max_iter
    {
        z = Complex::with_val(
            render_settings.prec,
            (
                &z.real().abs_ref().complete(render_settings.prec),
                &z.imag().abs_ref().complete(render_settings.prec),
            ),
        );
        z.pow_assign(2);
        z.add_assign(&p);

        n += 1;
    }

    if n == render_settings.max_iter {
        return -1;
    }

    n
}

pub(crate) const BURNING_SHIP: Fractal = Fractal {
    default_pos: (0.5, 0.5),
    get: &get_burning_ship,
    name: "BurningShip",
    details: concat!(
        // U_{n+1} = (|\Re(U_n)| + i |\Im(U_n)|)^2 + P
        "Formula: \n<acc Uₙ₊₁ = (|ℜ(Uₙ)| + i |ℑ(Uₙ)|)² + P>\n",
        "Where <acc U₀=0>, ",
        "and <acc P> is the complex number at the position of the pixel. ",
        "The vertical and horizontal axis are inverted (negatives on the top/right) in order for the render to look like a ship.",
    ),
};
