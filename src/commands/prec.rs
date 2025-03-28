use super::{command_increment::command_increment, Command};
use crate::AppState;

pub(crate) const MIN_DECIMAL_PREC: u32 = 8;
pub(crate) const MAX_DECIMAL_PREC: u32 = 65535;

pub(crate) fn execute_prec(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let val = command_increment(
        state,
        state.render_settings.prec,
        args,
        MIN_DECIMAL_PREC,
        MAX_DECIMAL_PREC,
    )?;
    if state.render_settings.wgpu_state.use_gpu {
        state.log_info(concat!(
            "Arbitrary precision if not taken into account when GPU mode is enabled. ",
            "You can disable GPU mode with the <command gpu> command."
        ));
    }
    state.set_decimal_prec(val);
    Ok(())
}
pub(crate) const PREC: Command = Command {
    execute: &execute_prec,
    name: "prec",
    aliases: &[],
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
