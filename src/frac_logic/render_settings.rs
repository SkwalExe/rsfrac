//! Contains the `RenderSettings` struct.

use image::ImageFormat;
use rand::{thread_rng, Rng};
use ratatui::style::Color;
use rug::{Complex, Float};

use crate::app_state::{HSLSettings, MAX_HSL_VALUE};
use crate::colors::{self, Palette, COLORS};
use crate::commands::max_iter::{MAX_MAX_ITER, MIN_MAX_ITER};
use crate::commands::prec::{MAX_DECIMAL_PREC, MIN_DECIMAL_PREC};
use crate::frac_logic::CanvasCoords;
use crate::fractals::FRACTALS;
use crate::helpers::{decrement_wrap, increment_wrap, void_fills, VoidFill};

use super::gpu_util::WgpuState;

const DF_PREC_GPU: u32 = 64;
const DF_MAX_ITER_GPU: i32 = 128;
const DF_PREC_CPU: u32 = 32;
const DF_MAX_ITER_CPU: i32 = 64;

const BLACK: Color = Color::Rgb(0, 0, 0);
const WHITE: Color = Color::Rgb(255, 255, 255);
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
    /// Whether or not to use the GPU for computations.
    pub(crate) use_gpu: bool,
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
            use_gpu: false,
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

impl RenderSettings {
    /// Returns the cell size so as to keep the complex plane height
    pub(crate) fn cell_size_from_height(&self, height: i32) -> Float {
        self.get_plane_height() / height
    }

    /// Returns the cell size so as to keep the complex plane width
    pub(crate) fn _cell_size_from_wid(&self, wid: i32) -> Float {
        self.get_plane_wid() / wid
    }

    pub(crate) fn increment_color_offset(&mut self) {
        let max = self.get_palette().colors.len() as i32 * self.smoothness;
        increment_wrap(&mut self.color_scheme_offset, max);
    }
    pub(crate) fn decrement_color_offset(&mut self) {
        let max = self.get_palette().colors.len() as i32 * self.smoothness;
        decrement_wrap(&mut self.color_scheme_offset, max);
    }
    /// Load the default settings for CPU mode when GPU init fails at startup.
    pub(crate) fn cpu_defaults(&mut self) {
        self.set_decimal_prec(DF_PREC_CPU);
        self.set_max_iter(DF_MAX_ITER_CPU);
    }
    /// Sets the decimal precision and update the precision of existing values.
    pub(crate) fn set_decimal_prec(&mut self, prec: u32) {
        // Make sure the precision remains within the fixed bounds.
        self.prec = MAX_DECIMAL_PREC.min(MIN_DECIMAL_PREC.max(prec));
        // Update the precision of existing numeric values.
        self.pos.set_prec(self.prec);
        self.cell_size.set_prec(self.prec);
    }
    /// Increment positively or negatively the maximum divergence
    pub(crate) fn increment_max_iter(&mut self, increment: i32) {
        let new_max_iter = self.max_iter.saturating_add(increment);
        self.set_max_iter(MIN_MAX_ITER.max(MAX_MAX_ITER.min(new_max_iter)));
    }

    pub(crate) fn set_max_iter(&mut self, max_iter: i32) {
        self.max_iter = max_iter
    }

    /// Changes the selected fractal. Will update the GPU render pipeline if GPU mode
    /// is enabled, if then an error is met, GPU mode will be disabled and an error message will be
    /// returned. Note that this method will never fail, even though it can return an error
    /// message.
    pub(crate) fn select_fractal(&mut self, frac_i: usize) -> Result<(), String> {
        self.frac_index = frac_i;
        if self.use_gpu {
            if let Err(err) = self.update_fractal_shader_sync(None) {
                self.use_gpu = false;
                return Err(format!(
                    "Disabling GPU mode because fractal shader could not be loaded: {err}"
                ));
            };
        }
        Ok(())
    }

    /// Returns the selected color palette.
    pub(crate) fn get_palette(&self) -> &'static Palette {
        &COLORS[self.palette_index]
    }

    /// Returns a color corresponding to the given iteration count, using
    /// the currently selected color palette or hsl mode.
    pub(crate) fn color_from_div(&self, diverg: &i32) -> Color {
        let palette = self.get_palette();
        let mut rng = thread_rng();
        let void_fills_ = void_fills();

        if *diverg == -1 {
            // Return void color

            return match void_fills_[self.void_fill_index] {
                VoidFill::Transparent => Color::Reset,
                VoidFill::Black => BLACK,
                VoidFill::White => WHITE,
                // Same as if the div was 0
                VoidFill::ColorScheme => self.color_from_div(&0),
                VoidFill::RGBNoise => Color::Rgb(
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                ),
                VoidFill::RedNoise => Color::Rgb(rng.gen_range(0..255), 0, 0),
                VoidFill::GreenNoise => Color::Rgb(0, rng.gen_range(0..255), 0),
                VoidFill::BlueNoise => Color::Rgb(0, 0, rng.gen_range(0..255)),
            };
        }
        // If hsl mode is disabled, get the color using the palette
        if !self.hsl_settings.enabled {
            return colors::palette_color(
                *diverg,
                self.color_scheme_offset,
                palette,
                self.smoothness,
            );
        }

        Color::from_hsl(
            // We must add 1 to prevent division by 0
            // And we multiply by 20 so that it is not too smooth
            (*diverg as f64 / (self.hsl_settings.smoothness as f64 + 1.0) * 20.0
                // The transifion from an offset of 100 and an offset of 0 should not
                // be visible, it should make a complete loop
                + self.hsl_settings.hue_offset as f64 * 3.6)                 
                // The hue should loop around 360
                % 360.0, 
            self.hsl_settings.saturation as f64 / MAX_HSL_VALUE as f64 * 100.0,
            self.hsl_settings.lum as f64 / MAX_HSL_VALUE as f64 * 100.0,
        )
    }
}
