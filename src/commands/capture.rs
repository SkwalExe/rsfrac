use std::sync::mpsc;

use super::Command;
use crate::{
    app::{ScreenshotMaster, ScreenshotSlave},
    helpers::Vec2,
    AppState,
};

pub(crate) fn execute_capture(state: &mut AppState, _args: Vec<&str>) {
    let (tx, rx) = mpsc::channel();
    let size = Vec2::new(1920, 1080);
    let screenshot = ScreenshotSlave::new(size.clone(), tx, &state.render_settings);
    let handle = ScreenshotSlave::start(screenshot);
    state.requested_jobs.push(ScreenshotMaster::new(size, rx, handle));
}

pub(crate) const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Takes a high quality screenshot of the canvas.",
};
