use std::collections::HashMap;
use std::time::Instant;

use crate::{app::App, app_state::Stats, helpers::Vec2};

impl App {
    /// Run the selected fractal algorithm for each canvas coord
    pub(crate) fn render_canvas(&mut self) {
        let before = Instant::now();
        if self.app_state.redraw_canvas {
            self.app_state.redraw_canvas = false;

            self.diverg_matrix = self.app_state.render_settings.get_diverg_matrix(&Vec2::new(
                self.app_state.render_settings.canvas_size.x,
                self.app_state.render_settings.canvas_size.y,
            ));
        }

        if self.app_state.repaint_canvas {
            // reset the stats
            self.app_state.stats = Stats::default();

            self.points = HashMap::default();
            self.app_state.repaint_canvas = false;
            let mut non_void_points = 0;

            for (y, line) in self.diverg_matrix.iter().enumerate() {
                let y: i32 = y.try_into().unwrap();
                for (x, diverg) in line.iter().enumerate() {
                    let x: i32 = x.try_into().unwrap();

                    let color = self.app_state.render_settings.color_from_div(diverg);
                    self.points
                        .entry(color)
                        .or_default()
                        .push((x.into(), y.into()));
                    if *diverg != -1 {
                        non_void_points += 1;
                        self.app_state.stats.avg_diverg += *diverg as f64;
                    }
                    if diverg > &self.app_state.stats.highest_diverg {
                        self.app_state.stats.highest_diverg = *diverg;
                    }
                }
            }
            self.app_state.stats.avg_diverg /= non_void_points as f64;
        }

        // Render the canvas again if for some reason a
        // redraw was requested during the first render
        if self.app_state.redraw_canvas {
            self.render_canvas();
        }

        self.app_state.stats.render_time = before.elapsed();
    }
}
