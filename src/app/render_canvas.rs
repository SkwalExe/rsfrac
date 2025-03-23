use std::collections::HashMap;
use std::time::Instant;

use futures::executor::block_on;

use crate::{app::App, app_state::Stats, helpers::Vec2};

impl App {
    /// Run the selected fractal algorithm for each canvas coord
    pub(crate) fn render_canvas(&mut self) {
        // Used to know how long the rendering took.
        let before = Instant::now();

        // Compute a new divergenve matrix if we are redrawing the canvas.
        if self.app_state.redraw_canvas {
            let size = Vec2::new(
                self.app_state.render_settings.canvas_size.x,
                self.app_state.render_settings.canvas_size.y,
            );

            // Render differently depending on whether or not GPU mode is enabled.
            if self.app_state.render_settings.wgpu_state.use_gpu {
                // In GPU mode, try to render with the GPU, and disable GPU mode in case of a
                // failure.
                self.diverg_matrix = match block_on(
                    self.app_state
                        .render_settings
                        .get_gpu_diverg_matrix_async(&size, None),
                ) {
                    // If the render was ok => use the result
                    Ok(res) => res,
                    // If the render failed:
                    Err(err) => {
                        // Disable GPU mode.
                        self.app_state.render_settings.wgpu_state.use_gpu = false;
                        self.app_state.log_error(format!(
                            "Disabling GPU mode, because the render failed with error: {err}",
                        ));
                        // Run render_canvas again, and ask to redraw.
                        self.render_canvas();
                        return;
                    }
                }
            } else {
                self.diverg_matrix = self.app_state.render_settings.get_diverg_matrix(&size);
            }

            // Avoid redrawing the canvas again on the next frame.
            self.app_state.redraw_canvas = false;
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
