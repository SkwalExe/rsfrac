use super::{command_increment::command_increment, Command};
use crate::AppState;

const MIN_SMOOTHNESS: i32 = 1;
const MAX_SMOOTHNESS: i32 = 100;

pub(crate) fn execute_smoothness(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let val = command_increment(
        state,
        state.render_settings.smoothness,
        args,
        MIN_SMOOTHNESS,
        MAX_SMOOTHNESS,
    )?;
    state.render_settings.smoothness = val;
    state.request_redraw();

    Ok(())
}
pub(crate) const SMOOTHNESS: Command = Command {
    execute: &execute_smoothness,
    name: "smoothness",
    aliases: &["sm"],
    accepted_arg_count: &[0, 1, 2],

    detailed_desc: Some(concat!(
        "<green Usage: <command +/- [value]>>\n",
        "<green Usage: <command [smoothness]>>\n",
        "<green Usage: <command [without args]>>\n",
        "- If no arguments are given, display the smoothness.\n",
        "- If a value is specified directly, set the smoothness to the given value.\n",
        "- If a value is specified alongside an operator, ",
        "increase of decrease the smoothness by the given value.\n",
        "<acc [smoothness]> must be a valid integer between <acc 1> and <acc 100>.",
    )),
    basic_desc: concat!(
        "Changes the smoothness of the color palette, ",
        "lower values will contrast the adjacent divergences ",
        "while higher values will look smoother (and better in my opinion)."
    ),
};
