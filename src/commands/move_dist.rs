use crate::AppState;

use super::{command_increment::command_increment, Command};
pub(crate) const MIN_MOVE_DIST: i32 = 1;
pub(crate) const MAX_MOVE_DIST: i32 = 100;

pub(crate) fn execute_move_dist(state: &mut AppState, args: Vec<&str>) {
    if let Some(val) = command_increment(state, state.move_dist, args, MIN_MOVE_DIST, MAX_MOVE_DIST)
    {
        state.move_dist = val;
    }
}
pub(crate) const MOVE_DIST: Command = Command {
    execute: &execute_move_dist,
    name: "move_dist",
    accepted_arg_count: &[0, 1, 2],

    detailed_desc: Some(concat!(
        "<green Usage: <command +/- [increment]>>\n",
        "<green Usage: <command [distance]>>\n",
        "<green Usage: <command [without args]>>\n",
        "- If no arguments are given, display the current move distance.\n",
        "- If a value is specified directly, set the move distance to the given value.\n",
        "- If a value is specified alongside an operator, ",
        "increase of decrease the move distance by the given value.\n",
        "<acc [distance]> must be a valid integer.",
    )),
    basic_desc: concat!(
        "Set or display the move distance, in cell units. ",
        "If the move distance is set to N, ",
        "you will move within the canvas by a distance of N cells."
    ),
};
