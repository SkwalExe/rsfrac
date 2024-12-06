use super::{capture::execute_capture, Command};
use crate::AppState;

pub(crate) fn execute_capture_fit(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    let scale = 4;
    let width = 1920 * scale;
    let height = 1080 * scale;
    execute_capture(state, vec![&format!("{width}"), &format!("{height}")])
}

pub(crate) const CAPTURE_HQ: Command = Command {
    execute: &execute_capture_fit,
    name: "capture_hq",
    aliases: &["chq"],
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Takes a <acc VERY> high quality screenshot.",
};
