use super::{command_increment::command_increment, Command};
pub const MIN_MAX_ITER: i32 = 8;
pub const MAX_MAX_ITER: i32 = 10000;

pub fn execute_max_iter(app: &mut crate::app::App, args: Vec<&str>) {
    if let Some(val) = command_increment(
        app,
        app.render_settings.max_iter,
        args,
        MIN_MAX_ITER,
        MAX_MAX_ITER,
    ) {
        app.render_settings.max_iter = val;
        app.redraw_canvas = true;
    }
}
pub const MAX_ITER: Command = Command {
    execute: &execute_max_iter,
    name: "max_iter",
    accepted_arg_count: &[0, 1, 2],

    detailed_desc: Some(concat!(
        "<green Usage: <command +/- [value]>>\n",
        "<green Usage: <command [max_iter]>>\n",
        "<green Usage: <command [without args]>>\n",
        "- If no arguments are given, display the current iteration limit.\n",
        "- If a value is specified directly, set the iteration limit to the given value.\n",
        "- If a value is specified alongside an operator, ",
        "increase of decrease the iteration limit by the given value.\n",
        "<acc [max_iter]> must be a valid integer.",
    )),
    basic_desc:
        "Change the iteration limit used to determine if a point is converging or diverging.",
};
