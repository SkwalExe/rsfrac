use rand::{thread_rng, Rng};
use rayon::prelude::*;
use ratatui::style::Color;
use std::sync::Mutex;
use tui_input::Input as TuiInput;
use tui_scrollview::ScrollViewState;

mod stats;
pub(crate) use stats::Stats;

use crate::{
    app::Screenshot,
    colors::{self, Palette},
    commands::{
        max_iter::{MAX_MAX_ITER, MIN_MAX_ITER},
        prec::{MAX_DECIMAL_PREC, MIN_DECIMAL_PREC},
    },
    components::{Canvas, Input, LogPanel},
    frac_logic::{CanvasCoords, RenderSettings},
    helpers::{void_fills, Focus, Vec2, VoidFill, ZoomDirection},
};

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
const BLACK: Color = Color::Rgb(0, 0, 0);
const WHITE: Color = Color::Rgb(255, 255, 255);

pub(crate) type DivergMatrix = Vec<Vec<i32>>;

impl AppState {
    /// Returns divergence lines from `first_line` to `last_line` included.
    pub(crate) fn get_diverg_lines(
        &self,
        size: &Vec2<i32>,
        first_line: i32,
        last_line: i32,
    ) -> DivergMatrix {
        // Get the canvas coordinates of each row
        let half_x = size.x / 2;
        let half_y = size.y / 2;
        let cell_size = self.render_settings.get_plane_wid() / size.x;
        let render_settings = &self.render_settings;

        (-half_y + first_line..=-half_y + last_line)
            .into_par_iter()
            .map(|y| {
                (-half_x..=-half_x + size.x)
                    .into_par_iter()
                    .map(|x| {
                        (render_settings.get_frac_clos())(
                            render_settings
                                .coord_to_c_with_cell_size(CanvasCoords::new(x, y), &cell_size),
                            render_settings,
                        )
                    })
                    .collect()
            })
            .collect()
    }
    /// Returns a divergence matrix of the specified size.
    pub(crate) fn get_diverg_matrix(&self, size: Vec2<i32>) -> DivergMatrix {
        let last_line = size.y - 1;
        self.get_diverg_lines(&size, 0, last_line)
    }
    pub(crate) fn get_palette(&self) -> &'static Palette {
        &colors::COLORS[self.palette_index]
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
}
