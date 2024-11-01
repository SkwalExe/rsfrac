use std::time::Duration;

pub(crate) struct Stats {
    pub(crate) avg_diverg: f64,
    pub(crate) highest_diverg: i32,
    /// The duration took by the latest full canvas rendering
    pub(crate) render_time: Duration,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            avg_diverg: 0.0,
            highest_diverg: 0,
            render_time: Default::default(),
        }
    }
}
