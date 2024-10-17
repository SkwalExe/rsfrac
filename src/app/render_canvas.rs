use std::collections::HashMap;
use std::time::Instant;

use crate::app::App;
use crate::colors;
use crate::stats::Stats;
use rand::{thread_rng, Rng};
use ratatui::style::Color;
use rayon::prelude::*;

use super::fractal_logic::CanvasCoords;
use super::{void_fills, VoidFill};

const BLACK: Color = Color::Rgb(0, 0, 0);
const WHITE: Color = Color::Rgb(255, 255, 255);

impl App {
    /// Run the selected fractal algorithm for each canvas coord
    pub fn render_canvas(&mut self) {
        let before = Instant::now();
        self.redraw_canvas = false;

        // reset the stats
        self.stats = Stats::default();

        self.points = HashMap::default();

        // Get the canvas coordinates of each row
        let half_x = self.render_settings.canvas_size.x / 2;
        let mod_x = self.render_settings.canvas_size.x % 2;
        let half_y = self.render_settings.canvas_size.y / 2;
        let mod_y = self.render_settings.canvas_size.y % 2;
        let lines: Vec<_> = (-half_y..half_y + mod_y).collect();

        let line_divergs: Vec<Vec<i32>> = lines
            .par_iter()
            .map(|y| {
                let points: Vec<_> = (-half_x..half_x + mod_x).collect();
                let point_divergs: Vec<i32> = points
                    .par_iter()
                    .map(|x| {
                        (self.render_settings.get_frac_clos())(
                            self.render_settings.coord_to_c(CanvasCoords::new(*x, *y)),
                            &self.render_settings,
                        )
                    })
                    .collect();
                point_divergs
            })
            .collect();

        let palette = self.get_palette();
        let mut non_void_points = 0;
        let mut rng = thread_rng();
        let void_fills_ = void_fills();
        for (y, line) in line_divergs.iter().enumerate() {
            let y: i32 = y.try_into().unwrap();
            for (x, diverg) in line.iter().enumerate() {
                let x: i32 = x.try_into().unwrap();

                let color = if *diverg == -1 {
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
                };

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
