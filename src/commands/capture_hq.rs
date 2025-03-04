use super::{capture::execute_capture, Command};
use crate::AppState;

pub(crate) fn execute_capture_fit(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let scale = 4;
    let width = 1920 * scale;
    let height = 1080 * scale;
    let width_str = format!("{width}");
    let height_str = format!("{height}");
    let mut capture_args: Vec<&str> = vec![&width_str, &height_str];

    if args.len() == 1 {
        capture_args.push(args[0]);
    }

    execute_capture(state, capture_args)
}

pub(crate) const CAPTURE_HQ: Command = Command {
    execute: &execute_capture_fit,
    name: "capture_hq",
    aliases: &["chq"],
    accepted_arg_count: &[0, 1],
    detailed_desc: None,
    basic_desc: concat!(
        "Takes a <acc VERY> high quality screenshot. ",
        "You can optionnaly provide a name for the screenshot."
    ),
};
