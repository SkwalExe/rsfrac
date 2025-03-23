//! Contains the `RenderSettings` methods.

use std::sync::mpsc::Sender;

use rand::{thread_rng, Rng};
use ratatui::style::Color;
use rug::Float;

use crate::app::SlaveMessage;
use crate::app_state::hsl_settings::MAX_HSL_VALUE;
use crate::colors::{self, Palette, COLORS};
use crate::commands::max_iter::{MAX_MAX_ITER, MIN_MAX_ITER};
use crate::commands::prec::{MAX_DECIMAL_PREC, MIN_DECIMAL_PREC};
use crate::helpers::{decrement_wrap, increment_wrap, void_fills, VoidFill};

use super::RenderSettings;

const DF_PREC_CPU: u32 = 32;
const DF_MAX_ITER_CPU: i32 = 64;

const BLACK: Color = Color::Rgb(0, 0, 0);
const WHITE: Color = Color::Rgb(255, 255, 255);

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
        if self.wgpu_state.use_gpu {
            if let Err(err) = self.wgpu_state.set_cs(self.get_frac_obj().name) {
                self.wgpu_state.use_gpu = false;
                return Err(format!(
                    "Disabling GPU mode because fractal shader could not be loaded: {err}"
                ));
            };
        }
        Ok(())
    }

    /// Initializes the WgpuState.
    pub(crate) async fn initialize_gpu(
        &mut self,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<(), String> {
        self.wgpu_state
            .initialize(self.get_frac_obj().name, sender)
            .await
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
            // I tried to implement a logarithmic scale, this is a draft implementation
            (*diverg as f64 / 10.0f64.powf(self.hsl_settings.smoothness as f64/30.0) * 30.0
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
