use super::Command;

pub fn execute_zoom_factor(app: &mut crate::app::App, args: Vec<&str>) {
    // If no args are provided, show the current positino
    if args.is_empty() {
        app.log_info_title(
            "Current Scaling Factor",
            format!("The scaling factor is set to <acc {}%>", app.scaling_factor),
        );
        return;
    }

    if let Ok(new_value) = args[0].parse::<i32>() {
        if new_value < 1 || new_value > 500 {
            app.log_error("Please, provide a value between 1 and 500.");
            return;
        }

        app.scaling_factor = new_value;
        app.log_success(format!(
            "Scaling factor successfully set to <acc {new_value}%>"
        ));
    } else {
        app.log_error("Please, provide a valid integer.")
    }
}

pub const ZOOM_FACTOR: Command = Command {
    execute: &execute_zoom_factor,
    name: "zoom_factor",
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
