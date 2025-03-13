//! Contains the `RenderSettings` struct.

use image::ImageFormat;
use rug::{Complex, Float};

use crate::app_state::hsl_settings::HSLSettings;
use crate::frac_logic::CanvasCoords;
use crate::fractals::FRACTALS;

use super::WgpuState;

const DF_PREC_GPU: u32 = 64;
const DF_MAX_ITER_GPU: i32 = 128;
const DEFAULT_JULIA_CONSTANT: (f32, f32) = (-9.9418604e-1, 2.61627e-1);
const DEFAULT_MANDEL_CONSTANT: (f32, f32) = (0.0, 0.0);
const DEFAULT_SMOOTHNESS: i32 = 7;
const DEFAULT_BAILOUT: f32 = 2.0;

/// Used to group values related to fractal rendering logic.
#[derive(Clone, Debug)]
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
    /// Must not be set directly!!!!!
    pub(crate) frac_index: usize,
    pub(crate) hsl_settings: HSLSettings,
    pub(crate) palette_index: usize,
    pub(crate) color_scheme_offset: i32,
    pub(crate) void_fill_index: usize,
    pub(crate) wgpu_state: WgpuState,
    pub(crate) image_format: ImageFormat,
    pub(crate) julia_constant: Complex,
    pub(crate) mandel_constant: Complex,
    pub(crate) bailout: f32,
    pub(crate) smoothness: i32,
    /// The limit of size (in lines) for a render pass.
    pub(crate) chunk_size_limit: Option<i32>,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            image_format: ImageFormat::Png,
            frac_index: Default::default(),
            pos: Complex::with_val(DF_PREC_GPU, FRACTALS[0].default_pos),
            max_iter: DF_MAX_ITER_GPU,
            cell_size: Float::new(DF_PREC_GPU),
            canvas_size: Default::default(),
            prec: DF_PREC_GPU,
            color_scheme_offset: Default::default(),
            palette_index: 4,
            void_fill_index: Default::default(),
            wgpu_state: WgpuState::default(),
            julia_constant: Complex::with_val(DF_PREC_GPU, DEFAULT_JULIA_CONSTANT),
            mandel_constant: Complex::with_val(DF_PREC_GPU, DEFAULT_MANDEL_CONSTANT),
            bailout: DEFAULT_BAILOUT,
            smoothness: DEFAULT_SMOOTHNESS,
            chunk_size_limit: None,
            hsl_settings: Default::default(),
        }
    }
}
