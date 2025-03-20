//! Contains the state of the application in display

use ratatui::style::Color;
use std::collections::HashMap;

mod events;
mod main_loop;
mod parallel_jobs;
mod render_app;
mod render_canvas;

pub(crate) use parallel_jobs::{ScreenshotMaster, SlaveMessage, WaitingScreenshot};
pub(crate) type CanvasPoints = HashMap<Color, Vec<(f64, f64)>>;

use crate::{
    frac_logic::DivergMatrix,
    helpers::{Chunks, Focus},
    AppState,
};

#[derive(Default)]
pub struct App {
    /// Points to render in the canvas.
    pub(crate) points: CanvasPoints,
    /// Area for each component to render into.
    pub(crate) chunks: Chunks,
    pub(crate) app_state: AppState,
    pub(crate) diverg_matrix: DivergMatrix,
    pub(crate) parallel_jobs: Vec<ScreenshotMaster>,
    /// Whether or not to render the command system sidebar on the screen.
    pub(crate) hide_sidepanel: bool,
}

impl App {
    pub(crate) fn toggle_sidepanel(&mut self) {
        self.hide_sidepanel = !self.hide_sidepanel;
        self.app_state.request_redraw();
        if self.hide_sidepanel {
            self.app_state.focused = Focus::Canvas;
        }
    }
}
