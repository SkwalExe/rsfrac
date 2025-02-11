use super::Command;
use crate::{helpers::markup::esc, AppState};

pub(crate) fn execute_zoom_factor(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    // If no args are provided, show the current positino
    if args.is_empty() {
        state.log_info_title(
            "Current Scaling Factor",
            format!(
                "The scaling factor is set to <acc {}%>",
                state.scaling_factor
            ),
        );
        return Ok(());
    }

    let new_value = args[0]
        .parse::<i32>()
        .map_err(|err| format!("Please provide a valid integer: {}", esc(err)))?;

    if !(1..=500).contains(&new_value) {
        return Err("Please, provide a value between 1 and 500.".to_string());
    }

    state.scaling_factor = new_value;
    state.log_success(format!(
        "Scaling factor successfully set to <acc {new_value}%>"
    ));
    Ok(())
}

pub(crate) const ZOOM_FACTOR: Command = Command {
    execute: &execute_zoom_factor,
    name: "zoom_factor",
    aliases: &["zf"],
    accepted_arg_count: &[0, 1],
    basic_desc: "View or set the scaling factor used when zooming in or out the canvas.",
    detailed_desc: Some(concat!(
        "<green Usage: <command [percentage]>>\n",
        "<green Usage: <command [without args]>>\n",
        "If no arguments are given, display the current scaling factor. ",
        "Else, set the scaling factor to the given argument. ",
        "The scaling factor is a number between ]1 and 500]"
    )),
};
