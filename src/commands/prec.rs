use super::{command_increment, Command};
pub const MIN_DECIMAL_PREC: u32 = 8;
pub const MAX_DECIMAL_PREC: u32 = 10000;

pub fn execute_prec(app: &mut crate::app::App, args: Vec<&str>) {
    if let Some(val) = command_increment(
        app,
        app.render_settings.prec,
        args,
        MIN_DECIMAL_PREC,
        MAX_DECIMAL_PREC,
    ) {
        app.render_settings.prec = val;
        app.redraw_canvas = true;
    }
}
pub const PREC: Command = Command {
    execute: &execute_prec,
    name: "prec",
    accepted_arg_count: &[0, 1, 2],

    detailed_desc: Some(concat!(
        "<green Usage: <command [precision]>>\n",
        "<green Usage: <command [without args]>>\n",
        "If no arguments are given, display the current precision. ",
        "Else, set the numeric precision to the given bit-length. ",
        "<acc [precision]> must be a valid integer.",
    )),
    basic_desc:
        "Fine-tune the numeric precision by specifying the desired bit length for numeric values.",
};
