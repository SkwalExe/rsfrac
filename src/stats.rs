use std::time::Duration;

pub struct Stats {
    pub avg_diverg: f64,
    pub highest_diverg: i32,
    pub render_time: Duration,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            avg_diverg: 0.0,
            highest_diverg: 0,
            render_time: Default::default()
        }
    }
}
