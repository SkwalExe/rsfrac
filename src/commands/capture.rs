use super::Command;
use crate::{app::WaitingScreenshot, helpers::Vec2, AppState};

pub(crate) fn execute_capture(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
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

        let parsed_height: i32 = height.parse().map_err(|err| format!("The provided height could not be parsed, make sure to enter a valid integer: {err}"))?;

        let size_range = 16..u16::MAX as i32 + 1;

        if !size_range.contains(&parsed_height) || !size_range.contains(&parsed_width) {
            return Err(
                "The screenshot must be at least 16 and at most 65535 pixels in width and height."
                    .to_string(),
            );
        }

        Vec2::new(parsed_width, parsed_height)
    };
    state.requested_jobs.push(WaitingScreenshot {
        size,
        rs: state.render_settings.clone(),
    });

    Ok(())
}

pub(crate) const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    aliases: &["cp"],
    accepted_arg_count: &[0, 2],
    detailed_desc: Some(concat!(
        "<green Usage: <command [width] [height]>>\n",
        "<green Usage: <command [without args]>>\n",
        "Take a screenshot with the specified size of the default of <acc 1920x1080>.",
    )),
    basic_desc: "Takes a high quality screenshot of the canvas.",
};
