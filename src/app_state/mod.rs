//! Contains the logic state of the application (can be headless).

use rug::{ops::CompleteRound, Complex, Float};
use std::{collections::HashMap, sync::Mutex};
use tui_input::Input as TuiInput;
use tui_scrollview::ScrollViewState;

mod click_modes;
mod stats;
pub(crate) use click_modes::{ClickConfig, ClickMode};
pub(crate) use stats::Stats;

use crate::{
    app::WaitingScreenshot,
    colors::get_palette_index_by_name,
    components::{Canvas, Input, LogPanel},
    frac_logic::{CanvasCoords, RenderSettings},
    fractals::get_frac_index_by_name,
    helpers::{void_fills, Focus, SavedState, ZoomDirection},
};

pub(crate) struct AppState {
    pub(crate) redraw_canvas: bool,
    pub(crate) repaint_canvas: bool,
    pub(crate) stats: Stats,
    pub(crate) focused: Focus,
    pub(crate) quit: bool,
    pub(crate) log_messages: Vec<String>,
    pub(crate) prioritized_log_messages: HashMap<i64, String>,
    pub(crate) log_panel_scroll_state: Mutex<ScrollViewState>,
    pub(crate) last_command: String,
    pub(crate) command_input: TuiInput,
    pub(crate) move_dist: i32,
    pub(crate) scaling_factor: i32,
    pub(crate) render_settings: RenderSettings,
    pub(crate) requested_jobs: Vec<WaitingScreenshot>,
    pub(crate) click_config: ClickConfig,
    pub(crate) remove_jobs: bool,
}

const DF_MOVE_DISTANCE_CPU: i32 = 8;
const DF_SCALING_FACTOR_CPU: i32 = 20;
const DF_MOVE_DISTANCE_GPU: i32 = 4;
const DF_SCALING_FACTOR_GPU: i32 = 8;

impl Default for AppState {
    fn default() -> Self {
        Self {
            stats: Default::default(),
            redraw_canvas: true,
            remove_jobs: false,
            repaint_canvas: true,
            last_command: String::new(),
            quit: false,
            focused: Default::default(),
            command_input: Default::default(),
            log_messages: Default::default(),
            prioritized_log_messages: Default::default(),
            log_panel_scroll_state: Default::default(),
            render_settings: Default::default(),
            scaling_factor: DF_SCALING_FACTOR_GPU,
            move_dist: DF_MOVE_DISTANCE_GPU,
            requested_jobs: Default::default(),
            click_config: Default::default(),
        }
    }
}

impl AppState {
    /// Load default settings for CPU mode when GPU init fails at startup
    pub(crate) fn cpu_defaults(&mut self) {
        self.move_dist = DF_MOVE_DISTANCE_CPU;
        self.scaling_factor = DF_SCALING_FACTOR_CPU;
        self.render_settings.cpu_defaults();
    }
    /// Loads the data from a rsf file.
    pub(crate) fn apply(&mut self, saved: SavedState, filename: &str) {
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
                    .map_err(|err| format!("Invalid canvas position: {err}"))?
                    .complete((self.render_settings.prec, self.render_settings.prec));
            }

            // Change the mandelbrot constant
            if let Some(c) = saved.mandel_constant {
                self.render_settings.mandel_constant = Complex::parse(c)
                    .map_err(|err| format!("Invalid mandelbrot constant: {err}"))?
                    .complete((self.render_settings.prec, self.render_settings.prec));
            }

            // Change the julia constant
            if let Some(c) = saved.julia_constant {
                self.render_settings.julia_constant = Complex::parse(c)
                    .map_err(|err| format!("Invalid julia constant: {err}"))?
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
                        .map_err(|err| format!("Invalid canvas width: {err}"))?
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

            Ok(())
        })();

        self.request_redraw();

        match result {
            Err(err) => self.log_error(format!(
                "Could not finish loading the state file (<command {filename}>) due to an error: <red {err}>"
            )),
            Ok(_) => self.log_success(format!(
                "Successfully loaded state from: <command {filename}>.",
            ))
        }
    }

    /// Only repaint the canvas without generating a new divergence matrix.
    pub(crate) fn request_repaint(&mut self) {
        self.repaint_canvas = true;
    }
    /// Update the divergence matrix and repaint the canvas.
    pub(crate) fn request_redraw(&mut self) {
        self.redraw_canvas = true;
        self.request_repaint();
    }
    /// Return the text to display in the footer
    pub(crate) fn footer_text(&self) -> &'static [&'static str] {
        match self.focused {
            Focus::LogPanel => LogPanel::FOOTER_TEXT,
            Focus::Canvas => Canvas::FOOTER_TEXT,
            Focus::Input => Input::FOOTER_TEXT,
        }
    }
    /// Increment positively or negatively the maximum divergence, and ask for canvas redraw
    pub(crate) fn increment_max_iter(&mut self, increment: i32) {
        self.render_settings.increment_max_iter(increment);
        self.request_redraw();
    }
    /// Increment positively or negatively the decimal precision,
    /// and update the precision of existing numeric values.
    pub(crate) fn increment_decimal_prec(&mut self, increment: i32) {
        let new_prec = self.render_settings.prec.saturating_add_signed(increment);
        self.set_decimal_prec(new_prec);
    }

    /// Sets the decimal precision and update the precision of existing values.
    pub(crate) fn set_decimal_prec(&mut self, prec: u32) {
        self.render_settings.set_decimal_prec(prec);
        // Ask for canvas redraw
        self.request_redraw();
    }
    pub(crate) fn zoom_at(&mut self, pos: CanvasCoords, direction: ZoomDirection) {
        let inintial_c_pos = self.render_settings.coord_to_c(pos.clone());
        self.zoom(direction);
        let new_c_pos = self.render_settings.coord_to_c(pos);

        self.render_settings.pos += inintial_c_pos - new_c_pos;
    }
    pub(crate) fn zoom(&mut self, direction: ZoomDirection) {
        let scaling_factor = 1.0 + self.scaling_factor as f64 / 100.0;

        match direction {
            ZoomDirection::In => self.render_settings.cell_size /= scaling_factor,
            ZoomDirection::Out => self.render_settings.cell_size *= scaling_factor,
        }
    }
}
