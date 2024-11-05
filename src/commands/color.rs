use super::Command;
use crate::colors::{get_palette_index_by_name, COLORS};
use crate::AppState;

pub(crate) fn execute_color(state: &mut AppState, args: Vec<&str>) {
    if args.is_empty() {
        state.log_raw(format!(
            "Current colors: <acc {}>\nAvailable colors: {}",
            state.render_settings.get_palette().name,
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
        None => state.log_error(format!("Could not find palette: <red {}>", args[0])),
        Some(pal) => {
            state.render_settings.palette_index = pal;
            state.log_success(format!("Selected color scheme: <acc {}>", COLORS[pal].name,));
            state.redraw_canvas = true
        }
    }
}

pub(crate) const COLOR: Command = Command {
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
