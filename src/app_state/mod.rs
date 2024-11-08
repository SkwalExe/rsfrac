use std::{collections::HashMap, sync::Mutex};
use tui_input::Input as TuiInput;
use tui_scrollview::ScrollViewState;

mod stats;
pub(crate) use stats::Stats;

use crate::{
    app::ScreenshotMaster,
    commands::{
        max_iter::{MAX_MAX_ITER, MIN_MAX_ITER},
        prec::{MAX_DECIMAL_PREC, MIN_DECIMAL_PREC},
    },
    components::{Canvas, Input, LogPanel},
    frac_logic::{CanvasCoords, RenderSettings},
    helpers::{Focus, ZoomDirection},
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
    pub(crate) command_input: TuiInput,
    pub(crate) marker: Option<CanvasCoords>,
    pub(crate) move_dist: i32,
    pub(crate) scaling_factor: i32,
    pub(crate) render_settings: RenderSettings,
    pub(crate) requested_jobs: Vec<ScreenshotMaster>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            stats: Default::default(),
            redraw_canvas: true,
            repaint_canvas: true,
            quit: false,
            focused: Default::default(),
            command_input: Default::default(),
            log_messages: Default::default(),
            prioritized_log_messages: Default::default(),
            log_panel_scroll_state: Default::default(),
            render_settings: Default::default(),
            scaling_factor: 20,
            move_dist: 8,
            marker: Default::default(),
            requested_jobs: Default::default(),
        }
    }
}

impl AppState {
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
        let new_max_iter = self.render_settings.max_iter.saturating_add(increment);
        self.render_settings.max_iter = MIN_MAX_ITER.max(MAX_MAX_ITER.min(new_max_iter));
        self.request_redraw();
    }
    /// Increment positively or negatively the decimal precision,
    /// and update the precision of existing numeric values.
    pub(crate) fn increment_decimal_prec(&mut self, increment: i32) {
        let new_prec = self.render_settings.prec.saturating_add_signed(increment);

        // Make sure the precision remains within the fixed bounds.
        self.render_settings.prec = MAX_DECIMAL_PREC.min(MIN_DECIMAL_PREC.max(new_prec));

        // Update the precision of existing numeric values.
        self.render_settings.pos.set_prec(self.render_settings.prec);
        self.render_settings
            .cell_size
            .set_prec(self.render_settings.prec);

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
