use crate::colors::{get_palette_index_by_name, COLORS};

use super::Command;

pub fn execute_color(app: &mut crate::app::App, args: Vec<&str>) {
    if args.is_empty() {
        app.log_raw(format!(
            "Current colors: <acc {}>\nAvailable colors: {}",
            app.get_palette().name,
            COLORS
                .iter()
                .map(|col| format!("<acc {}>", col.name))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        return;
    }

    let palette = get_palette_index_by_name(args[0]);
    match palette {
        None => app.log_error(format!("Could not find palette: <red {}>", args[0])),
        Some(pal) => {
            app.palette_index = pal;
            app.log_success(format!("Selected color scheme: <acc {}>", COLORS[pal].name,));
            app.redraw_canvas = true
        }
    }
}

pub const COLOR: Command = Command {
    execute: &execute_color,
    name: "color",
    accepted_arg_count: &[0, 1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [color]>>\n",
        "<green Usage: <command [without args]>>\n",
        "If no argument is given, display the available color schemes. ",
        "Else, select the specified color scheme.",
    )),
    basic_desc: "List available color schemes or select the specified one.",
};
