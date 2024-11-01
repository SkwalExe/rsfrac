use std::sync::Mutex;
use tui_input::Input as TuiInput;
use tui_scrollview::ScrollViewState;

use crate::{
    colors::{self, Palette},
    commands::{
        max_iter::{MAX_MAX_ITER, MIN_MAX_ITER},
        prec::{MAX_DECIMAL_PREC, MIN_DECIMAL_PREC},
    },
    components::{canvas::Canvas, input::Input, log_panel::LogPanel},
    helpers::{Focus, ZoomDirection},
};

use super::{
    fractal_logic::CanvasCoords, logging::VERSION, parallel_jobs::Screenshot, stats::Stats,
    RenderSettings,
};

const LOG_MESSAGE_LIMIT: usize = 500;

pub(crate) struct AppState {
    pub(crate) redraw_canvas: bool,
    pub(crate) stats: Stats,
    pub(crate) focused: Focus,
    pub(crate) quit: bool,
    pub(crate) log_messages: Vec<String>,
    pub(crate) log_panel_scroll_state: Mutex<ScrollViewState>,
    pub(crate) command_input: TuiInput,
    pub(crate) marker: Option<CanvasCoords>,
    pub(crate) move_dist: i32,
    pub(crate) scaling_factor: i32,
    pub(crate) palette_index: usize,
    pub(crate) color_scheme_offset: i32,
    pub(crate) void_fill_index: usize,
    pub(crate) render_settings: RenderSettings,
    pub(crate) requested_jobs: Vec<Screenshot>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            stats: Default::default(),
            redraw_canvas: true,
            quit: false,
            focused: Default::default(),
            command_input: Default::default(),
            log_messages: Default::default(),
            log_panel_scroll_state: Default::default(),
            render_settings: Default::default(),
            scaling_factor: 20,
            move_dist: 8,
            marker: Default::default(),
            color_scheme_offset: Default::default(),
            palette_index: 0,
            void_fill_index: Default::default(),
            requested_jobs: Default::default(),
        }
    }
}

impl AppState {
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
        self.redraw_canvas = true;
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
        self.redraw_canvas = true;
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
    pub(crate) fn log_raw(&mut self, message: impl Into<String>) {
        self.log_messages.push(message.into());
        if self.log_messages.len() > LOG_MESSAGE_LIMIT {
            self.log_messages.remove(0);
        }
        let state = &mut self.log_panel_scroll_state.lock().unwrap();
        state.scroll_to_bottom();
    }

    pub(crate) fn log_success_title(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.log_raw(format!("<bggreen  {} >\n{}", title.into(), message.into()))
    }
    pub(crate) fn log_success(&mut self, message: impl Into<String>) {
        self.log_success_title("Success", message.into())
    }
    pub(crate) fn log_info_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!("<bgacc  {} >\n{}", title.into(), message.into()))
    }
    pub(crate) fn log_info(&mut self, message: impl Into<String>) {
        self.log_info_title("Info", message.into())
    }

    pub(crate) fn log_error(&mut self, message: impl Into<String>) {
        self.log_error_title("Error", message);
    }
    pub(crate) fn log_error_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!(
            "<bgred  {} >\n<red {}>",
            title.into(),
            message.into()
        ))
    }

    /// Print the initial log messages
    pub(crate) fn initial_message(&mut self) {
        self.log_raw(format!(
            "<bgacc Welcome to Rsfrac v{VERSION}>\nAuthor: <acc Léopold Koprivnik>\nGithub Repo: <acc SkwalExe/rsfrac>",
        ));
        self.log_raw(
            "If you are experiencing slow rendering, try to reduce the size of your terminal.",
        );
        self.log_raw("You can switch between the canvas, the log panel and the command input using <acc tab>. Use the <acc help> command for more information.");
    }
    pub(crate) fn get_palette(&self) -> &'static Palette {
        &colors::COLORS[self.palette_index]
    }
}
