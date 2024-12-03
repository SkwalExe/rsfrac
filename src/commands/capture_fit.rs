use super::{capture::execute_capture, Command};
use crate::AppState;

fn width_from_height(height: i32, state: &AppState) -> i32 {
    (state.render_settings.canvas_size.x as f32 / state.render_settings.canvas_size.y as f32
        * height as f32) as i32
}

fn height_from_width(width: i32, state: &AppState) -> i32 {
    (state.render_settings.canvas_size.y as f32 / state.render_settings.canvas_size.x as f32
        * width as f32) as i32
}

pub(crate) fn execute_capture_fit(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        let width = 1920;
        let height = height_from_width(width, state);
        execute_capture(state, vec![&format!("{width}"), &format!("{height}")])
    } else {
        let dimension = ["height", "width"]
            .iter()
            .find(|n| n.starts_with(&args[0].to_lowercase()))
            .ok_or("The first argument must be <acc width> or <acc height>.")?;

        let parsed: i32 = args[1].parse().map_err(|_| {
            "Could not parse the specified size (second argument), make sure it is a valid integer."
        })?;

        if *dimension == "height" {
            let width = width_from_height(parsed, state);
            execute_capture(state, vec![&format!("{width}"), &format!("{parsed}")])
        } else {
            let height = height_from_width(parsed, state);
            execute_capture(state, vec![&format!("{parsed}"), &format!("{height}")])
        }
    }
}

pub(crate) const CAPTURE_FIT: Command = Command {
    execute: &execute_capture_fit,
    name: "capture_fit",
    aliases: &["cpf"],
    accepted_arg_count: &[0, 2],
    detailed_desc: Some(concat!(
        "<green Usage: <command [no args]>>\n",
        "Capture a screenshot 1920 pixels wide.\n",
        "<green Usage: <command [height/width] [size]>>\n",
        "Take a screenshot with the specified height or width.",
    )),
    basic_desc:
        "Takes a high quality screenshot while maintaining the same aspect ration as your canvas.",
};
