use rug::{ops::CompleteRound, Complex, Float};

use crate::{
    colors::get_palette_index_by_name,
    fractals::get_frac_index_by_name,
    helpers::{markup::esc, void_fills, SavedState},
    VERSION,
};

use super::AppState;
impl AppState {
    /// Loads the data from a rsf file.
    pub(crate) fn apply(&mut self, saved: SavedState, filename: &str) {
        // Check if the verison in the state file matches the current app version
        if saved.version.is_none() || saved.version.as_ref().unwrap() != VERSION {
            self.log_warn(format!(
                "The loaded state file was generated with Rsfrac version [<red {}>]. This should be OK but it is important to know, if you have any issues.",
                saved.version.unwrap_or("Unspecified".to_string())
            ));
        }

        let result = (|| -> Result<(), String> {
            // Change selected fractal
            if let Some(frac_name) = saved.frac_name {
                let res = self.render_settings.select_fractal(
                    get_frac_index_by_name(&frac_name)
                        .ok_or("Invalid fractal name in state file.")?,
                );

                self.handle_res(res);
            }

            // Change selected color palette
            if let Some(color_palette_name) = saved.color_palette_name {
                self.render_settings.palette_index = get_palette_index_by_name(&color_palette_name)
                    .ok_or("Invalid color palette name in state file.")?;
            }
            // Change the palette offset
            if let Some(palette_offset) = saved.palette_offset {
                self.render_settings.color_scheme_offset = palette_offset;
            }

            // Change the decimal precision
            if let Some(precision) = saved.precision {
                self.set_decimal_prec(precision);
            }

            // Change the smoothness
            if let Some(smoothness) = saved.smoothness {
                self.render_settings.smoothness = smoothness;
            }

            // Change the canvas position
            if let Some(pos) = saved.pos {
                self.render_settings.pos = Complex::parse(pos)
                    .map_err(|err| format!("Invalid canvas position: {}", esc(err)))?
                    .complete((self.render_settings.prec, self.render_settings.prec));
            }

            // Change the mandelbrot constant
            if let Some(c) = saved.mandel_constant {
                self.render_settings.mandel_constant = Complex::parse(c)
                    .map_err(|err| format!("Invalid mandelbrot constant: {}", esc(err)))?
                    .complete((self.render_settings.prec, self.render_settings.prec));
            }

            // Change the julia constant
            if let Some(c) = saved.julia_constant {
                self.render_settings.julia_constant = Complex::parse(c)
                    .map_err(|err| format!("Invalid julia constant: {}", esc(err)))?
                    .complete((self.render_settings.prec, self.render_settings.prec));
            }

            // Change the bailout
            if let Some(b) = saved.bailout {
                self.render_settings.bailout = b;
            }

            // Change the cell size
            if let Some(complex_width) = saved.complex_width {
                self.render_settings.set_width(
                    Float::parse(complex_width)
                        .map_err(|err| format!("Invalid canvas width: {}", esc(err)))?
                        .complete(self.render_settings.prec),
                );
            }

            // Change the max_iter value
            if let Some(max_iter) = saved.max_iter {
                self.render_settings.max_iter = max_iter;
            }

            // Change the void fill method
            if let Some(void_fill) = saved.void_fill {
                self.render_settings.void_fill_index = void_fills()
                    .iter()
                    .position(|vf| *vf == void_fill)
                    .ok_or("Invalid void fill name in state file.")?;
            }

            // Enable or disable hsl mode
            if let Some(hsl_enabled) = saved.hsl_mode {
                self.render_settings.hsl_settings.enabled = hsl_enabled
            }

            // Change the hsl smoothness
            if let Some(smoothness) = saved.hsl_smoothness {
                self.render_settings.hsl_settings.smoothness = smoothness;
            }

            // Change the hsl saturation
            if let Some(sat) = saved.hsl_saturation {
                self.render_settings.hsl_settings.saturation = sat;
            }
            // Change the hsl lum
            if let Some(lum) = saved.hsl_lum {
                self.render_settings.hsl_settings.lum = lum;
            }
            // Change the hsl hue offset
            if let Some(hue_offset) = saved.hsl_hue_offset {
                self.render_settings.hsl_settings.hue_offset = hue_offset;
            }

            Ok(())
        })();

        self.request_redraw();

        match result {
            Err(err) => self.log_error(format!(
                "Could not finish loading the state file (<command {filename}>) due to an error: <red {err}>"
            )),
            Ok(_) => self.log_success(format!(
                "Successfully loaded state from: <command {}>.", esc(filename),
            ))
        }
    }
}
