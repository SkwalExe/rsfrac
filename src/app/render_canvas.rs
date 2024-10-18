use std::collections::HashMap;
use std::time::Instant;

use crate::app::App;
use crate::colors::{self};
use crate::helpers::Vec2;
use crate::stats::Stats;
use rand::{thread_rng, Rng};
use ratatui::style::Color;
use rayon::prelude::*;

use super::fractal_logic::CanvasCoords;
use super::{void_fills, VoidFill};

const BLACK: Color = Color::Rgb(0, 0, 0);
const WHITE: Color = Color::Rgb(255, 255, 255);

pub type DivergMatrix = Vec<Vec<i32>>;
impl App {
    pub fn color_from_div(&self, diverg: &i32) -> Color {
        let palette = self.get_palette();
        let mut rng = thread_rng();
        let void_fills_ = void_fills();

        if *diverg == -1 {
            // Return void color

            match void_fills_[self.render_settings.void_fill_index] {
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
    pub fn get_diverg_matrix(&self, size: Vec2<i32>) -> DivergMatrix {
        // Get the canvas coordinates of each row
        let half_x = size.x / 2;
        let mod_x = size.x % 2;
        let half_y = size.y / 2;
        let mod_y = size.y % 2;
        let cell_size = self.get_plane_wid() / size.x;
        let render_settings = &self.render_settings;

        (-half_y..half_y + mod_y)
            .into_par_iter()
            .map(|y| {
                (-half_x..half_x + mod_x)
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
    /// Run the selected fractal algorithm for each canvas coord
    pub fn render_canvas(&mut self) {
        let before = Instant::now();
        self.redraw_canvas = false;

        // reset the stats
        self.stats = Stats::default();

        self.points = HashMap::default();

        let line_divergs = self.get_diverg_matrix(Vec2::new(
            self.render_settings.canvas_size.x,
            self.render_settings.canvas_size.y,
        ));

        let mut non_void_points = 0;

        for (y, line) in line_divergs.iter().enumerate() {
            let y: i32 = y.try_into().unwrap();
            for (x, diverg) in line.iter().enumerate() {
                let x: i32 = x.try_into().unwrap();

                let color = self.color_from_div(diverg);
                self.points
                    .entry(color)
                    .or_default()
                    .push((x.into(), y.into()));
                if *diverg != -1 {
                    non_void_points += 1;
                    self.stats.avg_diverg += *diverg as f64;
                }
                if diverg > &self.stats.highest_diverg {
                    self.stats.highest_diverg = *diverg;
                }
            }
        }

        self.stats.avg_diverg /= non_void_points as f64;

        // Render the canvas again if for some reason a
        // redraw was requested during the first render
        if self.redraw_canvas {
            self.render_canvas();
        }

        self.stats.render_time = before.elapsed();
    }
}
