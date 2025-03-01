//! Contains the Julia set rendering information.

use std::ops::AddAssign;

use rug::ops::{CompleteRound, PowAssign};
use rug::Complex;

use crate::frac_logic::RenderSettings;
use crate::fractals::Fractal;

/// Implement the formula for the julia set,
fn get_julia(mut p: Complex, render_settings: &RenderSettings) -> i32 {
    // iteration counter
    let mut n: i32 = 0;

    while *p
        .abs_ref()
        .complete((render_settings.prec, render_settings.prec))
        .real()
        < 4
        && n < render_settings.max_iter
    {
        p.pow_assign(2);
        p.add_assign(&render_settings.julia_constant);
        n += 1;
    }

    if n == render_settings.max_iter {
        return -1;
    }

    n
}

pub(crate) const JULIA: Fractal = Fractal {
    default_pos: (0.0, 0.0),
    get: &get_julia,
    name: "Julia",
    details: concat!(
        "Formula: \n<acc Uₙ₊₁ = Uₙ² + C >\n",
        // Todo: allow custom exp
        "Where <acc U₀> is the complex number at the position of the pixel, ",
        "and <acc C> is a constant that can be modified.\n",
    ),
};
