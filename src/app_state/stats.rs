use rug::Float;
use std::time::Duration;

use crate::frac_logic::RenderSettings;

/// Groups statistics about the last render pass.
#[derive(Default, Debug, Clone)]
pub(crate) struct Stats {
    /// The average divergence.
    pub(crate) avg_diverg: f64,
    /// The highest divergence.
    pub(crate) highest_diverg: i32,
    /// The duration took by the latest full canvas rendering
    pub(crate) render_time: Duration,
}

impl RenderSettings {
    /// Return the total number of points to render in the canvas (for stats).
    pub(crate) fn point_count(&self) -> i32 {
        self.canvas_size.x * self.canvas_size.y
    }
    /// Returns the global scaling factor of the canvas (for stats).
    pub(crate) fn get_zoom(&self) -> Float {
        self.get_default_cell_size() / &self.cell_size
    }
}
