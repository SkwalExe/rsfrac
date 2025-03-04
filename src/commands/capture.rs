use super::Command;
use crate::{
    app::WaitingScreenshot,
    helpers::{markup::esc, Vec2},
    AppState,
};

pub(crate) fn execute_capture(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    // No name by default
    let mut name = None;
    // default size
    let mut size = Vec2::new(1920, 1080);
    match args.len() {
        // If there is only one argument: it is the name
        1 => name = Some(args[0].to_string()),
        2 | 3 => {
            // If there are two or three arguments, parse the first two as the size
            let width = args[0];
            let height = args[1];

            let parsed_width = width.parse().map_err(|err| {
                format!(
                    "The provided width could not be parsed, make sure to enter a valid integer: {}",
                    esc(err)
                )
            })?;

            let parsed_height: i32 = height.parse().map_err(|err| {
                format!(
                    "The provided height could not be parsed, make sure to enter a valid integer: {}",
                    esc(err)
                )
            })?;

            let size_range = 16..u16::MAX as i32 + 1;

            if !size_range.contains(&parsed_height) || !size_range.contains(&parsed_width) {
                return Err(
                    "The screenshot must be at least 16 and at most 65535 pixels in width and height."
                        .to_string(),
                );
            }

            size = Vec2::new(parsed_width, parsed_height);

            // If there is a third argument, it is the capture name
            if args.len() == 3 {
                name = Some(args[2].to_string())
            }
        }
        _ => {}
    }

    state.requested_jobs.push(WaitingScreenshot {
        size,
        name,
        rs: state.render_settings.clone(),
    });

    Ok(())
}

pub(crate) const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    aliases: &["cp"],
    accepted_arg_count: &[0, 1, 2, 3],
    detailed_desc: Some(concat!(
        "<green Usage: <command [width] [height] [?name]>>\n",
        "<green Usage: <command [?name]>>\n",
        "Take a screenshot with the specified size or the default of <acc 1920x1080>. ",
        "The order of the arguments must match the examples above.",
    )),
    basic_desc: "Takes a high quality screenshot of the canvas.",
};
