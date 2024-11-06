use std::sync::mpsc;

use super::Command;
use crate::{
    app::{ScreenshotMaster, ScreenshotSlave},
    helpers::Vec2,
    AppState,
};

pub(crate) fn execute_capture(state: &mut AppState, args: Vec<&str>) {
    let (tx, rx) = mpsc::channel();

    let size = if args.len() == 0 {
        Vec2::new(1920, 1080)
    } else {
        let width = args[0];
        let height = args[1];

        let parsed_width = match width.parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                state.log_error(
                    "The provided width could not be parsed, make sure to enter a valid integer.",
                );
                return;
            }
        };
        let parsed_height = match height.parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                state.log_error(
                    "The provided height could not be parsed, make sure to enter a valid integer.",
                );
                return;
            }
        };

        if parsed_height < 16 || parsed_width < 16 {
            state.log_error("The screenshot must be at least 16 pixels in width and height.");
            return;
        }

        Vec2::new(parsed_width, parsed_height)
    };

    let screenshot = ScreenshotSlave::new(size.clone(), tx, &state.render_settings);
    let handle = ScreenshotSlave::start(screenshot);
    state
        .requested_jobs
        .push(ScreenshotMaster::new(size, rx, handle));
}

pub(crate) const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    accepted_arg_count: &[0, 2],
    detailed_desc: Some(concat!(
        "<green Usage: <command [width] [height]>>\n",
        "<green Usage: <command [without args]>>\n",
        "Take a screenshot with the specified size of the default of <acc 1920x1080>.",
    )),
    basic_desc: "Takes a high quality screenshot of the canvas.",
};
