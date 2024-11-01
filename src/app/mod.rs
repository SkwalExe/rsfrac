mod app_state;
mod build_chunks;
mod events;
mod fractal_logic;
mod logging;
mod main_loop;
mod parallel_jobs;
mod render_app;
mod render_canvas;
mod render_settings;
mod stats;

pub(crate) use app_state::AppState;
pub(crate) use render_settings::RenderSettings;
pub(crate) use logging::VERSION;
pub(crate) use parallel_jobs::Screenshot;

use crate::helpers::Chunks;

use strum::{Display, EnumIter, IntoEnumIterator};

use ratatui::style::Color;
use std::collections::HashMap;

#[derive(EnumIter, Debug, Display)]
pub(crate) enum VoidFill {
    Transparent,
    Black,
    White,
    GreenNoise,
    BlueNoise,
    RedNoise,
    RGBNoise,
    ColorScheme,
}

// Todo: find a way to make this a constant
pub(crate) fn void_fills() -> Vec<VoidFill> {
    VoidFill::iter().collect()
}

pub(crate) type CanvasPoints = HashMap<Color, Vec<(f64, f64)>>;
#[derive(Default)]
pub struct App {
    pub(crate) points: CanvasPoints,
    pub(crate) chunks: Chunks,
    pub(crate) app_state: AppState,
    pub(crate) parallel_jobs: Vec<Screenshot>,
}

