use crate::app::AppState;

use super::{command_increment::command_increment, Command};
pub(crate) const MIN_DECIMAL_PREC: u32 = 8;
pub(crate) const MAX_DECIMAL_PREC: u32 = 10000;

pub(crate) fn execute_prec(app_state: &mut AppState, args: Vec<&str>) {
    if let Some(val) = command_increment(
        app_state,
        app_state.render_settings.prec,
        args,
        MIN_DECIMAL_PREC,
        MAX_DECIMAL_PREC,
    ) {
        app_state.render_settings.prec = val;
        app_state.redraw_canvas = true;
    }
}
pub(crate) const PREC: Command = Command {
    execute: &execute_prec,
    name: "prec",
    accepted_arg_count: &[0, 1, 2],

    detailed_desc: Some(concat!(
        "<green Usage: <command +/- [increment]>>\n",
        "<green Usage: <command [precision]>>\n",
        "<green Usage: <command [without args]>>\n",
        "- If no arguments are given, display the current precision.\n",
        "- If a value is specified directly, set the precision to the given bit-length.\n",
        "- If a value is specified alongside an operator, ",
        "increase of decrease the move precision by the given value.\n",
        "<acc [precision]> must be a valid integer.",
    )),
    basic_desc:
        "Fine-tune the numeric precision by specifying the desired bit length for numeric values.",
};
