use super::Command;
use crate::{app::Screenshot, helpers::Vec2, AppState};

pub(crate) fn execute_capture(state: &mut AppState, _args: Vec<&str>) {
    state
        .requested_jobs
        .push(Screenshot::new(Vec2::new(1920, 1080)))
}

pub(crate) const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Takes a high quality screenshot of the canvas.",
};
