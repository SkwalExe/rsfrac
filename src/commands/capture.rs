use std::sync::mpsc;

use super::Command;
use crate::{
    app::{ScreenshotMaster, ScreenshotSlave},
    helpers::Vec2,
    AppState,
};

pub(crate) fn execute_capture(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();

    let size = if args.is_empty() {
        Vec2::new(1920, 1080)
    } else {
        let width = args[0];
        let height = args[1];

        let parsed_width = width.parse().map_err(|err| {
            format!(
                "The provided width could not be parsed, make sure to enter a valid integer: {err}"
            )
        })?;
        let parsed_height = height.parse().map_err(|err| format!("The provided height could not be parsed, make sure to enter a valid integer: {err}"))?;

        if parsed_height < 16 || parsed_width < 16 {
            return Err(
                "The screenshot must be at least 16 pixels in width and height.".to_string(),
            );
        }

        Vec2::new(parsed_width, parsed_height)
    };

    let screenshot = ScreenshotSlave::new(size.clone(), tx, &state.render_settings);
    let handle = ScreenshotSlave::start(screenshot);
    let master = ScreenshotMaster::new(size, rx, handle, state.render_settings.get_frac_obj().name);
    state
        .prioritized_log_messages
        .insert(master.id, String::from("Starting screenshot..."));
    state
        .log_panel_scroll_state
        .lock()
        .unwrap()
        .scroll_to_bottom();
    state.requested_jobs.push(master);

    Ok(())
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
