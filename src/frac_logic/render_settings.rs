//! Contains the `RenderSettings` struct.

use rand::{thread_rng, Rng};
use ratatui::style::Color;
use rug::{Complex, Float};

use crate::colors::{self, Palette, COLORS};
use crate::frac_logic::CanvasCoords;
use crate::fractals::FRACTALS;
use crate::helpers::{void_fills, VoidFill};

const DEFAULT_PREC: u32 = 32;
const DEFAULT_MAX_ITER: i32 = 32;

const BLACK: Color = Color::Rgb(0, 0, 0);
const WHITE: Color = Color::Rgb(255, 255, 255);

/// Used to group values related to fractal rendering logic.
#[derive(Clone)]
pub(crate) struct RenderSettings {
    /// The size of one canvas cell.
    pub(crate) cell_size: Float,
    /// The position of the middle of the canvas in the complex plane.
    pub(crate) pos: Complex,
    /// The size of the canvas in Canvas Coordinates.
    pub(crate) canvas_size: CanvasCoords,
    /// The decimal precision (bit-length) used for calculations.
    pub(crate) prec: u32,
    /// The maximum number of iterations before assuming that a point diverges.
    pub(crate) max_iter: i32,
    /// The index of the currently selected fractal.
    pub(crate) frac_index: usize,
    pub(crate) palette_index: usize,
    pub(crate) color_scheme_offset: i32,
    pub(crate) void_fill_index: usize,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            frac_index: Default::default(),
            pos: Complex::with_val(DEFAULT_PREC, FRACTALS[0].default_pos),
            max_iter: DEFAULT_MAX_ITER,
            cell_size: Float::new(DEFAULT_PREC),
            canvas_size: Default::default(),
            prec: DEFAULT_PREC,
            color_scheme_offset: Default::default(),
            palette_index: 0,
            void_fill_index: Default::default(),
        }
    }
}

impl RenderSettings {
    pub(crate) fn get_palette(&self) -> &'static Palette {
        &COLORS[self.palette_index]
    }
    pub(crate) fn color_from_div(&self, diverg: &i32) -> Color {
        let palette = self.get_palette();
        let mut rng = thread_rng();
        let void_fills_ = void_fills();

        if *diverg == -1 {
            // Return void color

            match void_fills_[self.void_fill_index] {
                VoidFill::Transparent => Color::Reset,
                VoidFill::Black => BLACK,
                VoidFill::White => WHITE,
                VoidFill::ColorScheme => {
                    colors::palette_color(*diverg + self.color_scheme_offset, palette)
                }
                VoidFill::RGBNoise => Color::Rgb(
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                ),
                VoidFill::RedNoise => Color::Rgb(rng.gen_range(0..255), 0, 0),
                VoidFill::GreenNoise => Color::Rgb(0, rng.gen_range(0..255), 0),
                VoidFill::BlueNoise => Color::Rgb(0, 0, rng.gen_range(0..255)),
            }
        } else {
            colors::palette_color(*diverg + self.color_scheme_offset, palette)
        }
    }
}