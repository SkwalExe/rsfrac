use ratatui::style::Color;
use std::collections::HashMap;

mod events;
mod main_loop;
mod parallel_jobs;
mod render_app;
mod render_canvas;

use crate::{helpers::Chunks, AppState};
pub(crate) use parallel_jobs::Screenshot;

pub(crate) type CanvasPoints = HashMap<Color, Vec<(f64, f64)>>;

#[derive(Default)]
pub struct App {
    /// Points to render in the canvas.
    pub(crate) points: CanvasPoints,
    /// Area for each component to render into.
    pub(crate) chunks: Chunks,
    pub(crate) app_state: AppState,
    pub(crate) parallel_jobs: Vec<Screenshot>,
}
