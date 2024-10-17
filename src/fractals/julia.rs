use std::ops::AddAssign;

use rug::ops::{CompleteRound, PowAssign};
use rug::Complex;

use crate::app::RenderSettings;
use crate::fractals::Fractal;

/// Implement the formula for the julia set,
fn get_julia(mut p: Complex, render_settings: &RenderSettings) -> i32 {
    // iteration counter
    let mut n: i32 = 0;

    // Todo: allow to change this
    let julia_cst = Complex::with_val(render_settings.prec, (-1, 0));

    while *p
        .abs_ref()
        .complete((render_settings.prec, render_settings.prec))
        .real()
        < 4
        && n < render_settings.max_iter
    {
        p.pow_assign(2);
        p.add_assign(&julia_cst);
        n += 1;
    }

    if n == render_settings.max_iter {
        return -1;
    }

    n
}

pub const JULIA: Fractal = Fractal {
    default_pos: (0.0, 0.0),
    get: &get_julia,
    name: "Julia",
    details: concat!(
        "Default Formula: \n<acc Uₙ₊₁ = Uₙ²-1 >\n",
        // Todo: allow custom exp
        "General case: \n<acc Uₙ₊₁ = Uₙ²+C >\n",
        "Where <acc U₀> is the complex number at the position of the pixel, ",
        "and <acc C> is a constant that can be modified, set to <acc -1+0j> by default.\n",
    ),
};
