use super::{command_increment::command_increment, Command};
use crate::AppState;

pub(crate) const MIN_MAX_ITER: i32 = 8;
pub(crate) const MAX_MAX_ITER: i32 = 10000;

pub(crate) fn execute_max_iter(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if let Some(val) = command_increment(
        state,
        state.render_settings.max_iter,
        args,
        MIN_MAX_ITER,
        MAX_MAX_ITER,
    ) {
        state.render_settings.max_iter = val;
        state.request_redraw();
    }
    Ok(())
}
pub(crate) const MAX_ITER: Command = Command {
    execute: &execute_max_iter,
    name: "max_iter",
    aliases: &["mi"],
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
