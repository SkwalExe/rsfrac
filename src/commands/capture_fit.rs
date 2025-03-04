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
    // default height and  width values
    let mut width = 1920;
    let mut height = height_from_width(width, state);

    // The filename to save the screenshot and the app state
    let mut name = None;

    // If there are 1 or 3 args, the last one is the file name
    if args.len() == 1 || args.len() == 3 {
        // we can unwrap because we know the len is not 0
        name = Some(args.last().unwrap());
    }

    // If there are 2 or 3 arguments, the first two are the size
    if args.len() == 2 || args.len() == 3 {
        let dimension = ["height", "width"]
            .iter()
            .find(|n| n.starts_with(&args[0].to_lowercase()))
            .ok_or("The first argument must be <acc width> or <acc height>.")?;

        let parsed: i32 = args[1].parse().map_err(|_| {
            "Could not parse the specified size (second argument), make sure it is a valid integer."
        })?;

        if *dimension == "height" {
            height = parsed;
            width = width_from_height(parsed, state);
        } else {
            width = parsed;
            height = height_from_width(parsed, state);
        }
    }

    let height_str = format!("{height}");
    let width_str = format!("{width}");

    // arguments passed to the capture command
    let mut capture_args: Vec<&str> = vec![&width_str, &height_str];

    if let Some(name) = name {
        capture_args.push(name);
    }

    execute_capture(state, capture_args)
}

pub(crate) const CAPTURE_FIT: Command = Command {
    execute: &execute_capture_fit,
    name: "capture_fit",
    aliases: &["cpf"],
    accepted_arg_count: &[0, 1, 2, 3],
    detailed_desc: Some(concat!(
        "<green Usage: <command [?name]>>\n",
        "Capture a screenshot 1920 pixels wide.\n",
        "<green Usage: <command [height/width] [size] [?name]>>\n",
        "Take a screenshot with the specified height or width.",
    )),
    basic_desc:
        "Takes a high quality screenshot while maintaining the same aspect ratio as your canvas.",
};
