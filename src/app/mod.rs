mod build_chunks;
mod events;
pub mod fractal_logic;
pub mod logging;
mod main_loop;
mod render_app;
mod render_canvas;
use std::{collections::HashMap, sync::Mutex};

use fractal_logic::CanvasCoords;
use ratatui::style::Color;
use rug::{Complex, Float};
use tui_input::Input as TuiInput;
use tui_scrollview::ScrollViewState;

use crate::{
    colors::{self, Palette},
    commands::{
        create_commands, max_iter::{MAX_MAX_ITER, MIN_MAX_ITER}, prec::{MAX_DECIMAL_PREC, MIN_DECIMAL_PREC}, Command
    },
    components::{canvas::Canvas, input::Input, log_panel::LogPanel},
    helpers::{Chunks, Focus},
    stats::Stats,
};
pub const DEFAULT_PREC: u32 = 32;
pub const DEFAULT_MAX_ITER: i32 = 32;
pub const DEFAULT_POS: (f64, f64) = (-0.5, 0.0);

pub struct RenderSettings {
    pub cell_size: Float,
    pub pos: Complex,
    // The size of the canvas in Canvas Coordinates
    pub canvas_size: CanvasCoords,
    pub prec: u32,
    pub max_iter: i32,
}

pub struct App {
    pub commands: HashMap<&'static str, &'static Command>,
    pub focused: Focus,
    pub quit: bool,
    pub marker: Option<CanvasCoords>,
    pub points: HashMap<Color, Vec<(f64, f64)>>,
    pub redraw_canvas: bool,
    pub move_dist: i32,
    pub chunks: Chunks,
    pub stats: Stats,
    pub command_input: TuiInput,
    pub palette_index: usize,
    pub log_messages: Vec<String>,
    pub app_state: Mutex<AppState>,
    pub render_settings: RenderSettings,
    /// The duration took by the latest full canvas rendering
    pub scaling_factor: i32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            max_iter: DEFAULT_MAX_ITER,
            pos: Complex::with_val(DEFAULT_PREC, DEFAULT_POS),
            cell_size: Float::new(DEFAULT_PREC),
            canvas_size: Default::default(),
            prec: DEFAULT_PREC,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            scaling_factor: 20,
            palette_index: 0,
            commands: create_commands(),
            focused: Default::default(),
            quit: false,
            points: Default::default(),
            redraw_canvas: true,
            chunks: Default::default(),
            move_dist: 8,
            stats: Default::default(),
            command_input: Default::default(),
            log_messages: Default::default(),
            app_state: Default::default(),
            render_settings: Default::default(),
            marker: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub log_panel_scroll_state: ScrollViewState,
}

impl App {
    /// Return the text to display in the footer
    pub fn footer_text(&self) -> &'static [&'static str] {
        match self.focused {
            Focus::LogPanel => LogPanel::FOOTER_TEXT,
            Focus::Canvas => Canvas::FOOTER_TEXT,
            Focus::Input => Input::FOOTER_TEXT,
        }
    }

    /// Return a complex corresponding to the default position,
    /// with the decimal precision configured on the App.
    pub fn get_default_pos(&self) -> Complex {
        Complex::with_val(self.render_settings.prec, DEFAULT_POS)
    }

    /// Increment positively or negatively the decimal precision, 
    /// and update the precision of existing numeric values.
    pub fn increment_decimal_prec(&mut self, increment: i32) {
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
    /// Increment positively or negatively the maximum divergence, and ask for canvas redraw
    pub fn increment_max_iter(&mut self, increment: i32) {
        let new_max_iter = self.render_settings.max_iter.saturating_add(increment);
        self.render_settings.max_iter = MIN_MAX_ITER.max(MAX_MAX_ITER.min(new_max_iter));
        self.redraw_canvas = true;
    }

    /// Return the total number of points to render in the canvas
    pub fn point_count(&self) -> i32 {
        self.render_settings.canvas_size.x * self.render_settings.canvas_size.y
    }

    /// Set the position of the canvas to the default position
    pub fn reset_pos(&mut self) {
        self.render_settings.pos = self.get_default_pos();
    }

    /// Set the cell size so that the total width of the canvas is 4 on the real axis
    pub fn reset_cell_size(&mut self) {
        self.render_settings.cell_size = self.get_default_cell_size();
    }

    /// Return the cell size so that the total width of the canvas is 4 on the real axis
    pub fn get_default_cell_size(&self) -> Float {
        Float::with_val(self.render_settings.prec, 4) / self.render_settings.canvas_size.x
    }

    pub fn get_zoom(&self) -> Float {
        self.get_default_cell_size() / &self.render_settings.cell_size
    }

    pub fn get_palette(&self) -> &'static Palette {
        &colors::COLORS[self.palette_index]
    }
}
