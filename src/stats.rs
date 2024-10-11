pub struct Stats {
    pub avg_diverg: f64,
    pub highest_diverg: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            avg_diverg: 0.0,
            highest_diverg: 0,
        }
    }
}
