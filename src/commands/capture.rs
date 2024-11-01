use crate::app::{AppState, Screenshot};

use super::Command;

pub(crate) fn execute_capture(app_state: &mut AppState, _args: Vec<&str>) {
    app_state.requested_jobs.push(Screenshot::new())
}

pub(crate) const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    accepted_arg_count: &[0],

    detailed_desc: Some(concat!("TODO",)),
    basic_desc: "TODO",
};
