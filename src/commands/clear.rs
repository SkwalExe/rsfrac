use super::Command;
pub fn execute_clear(app: &mut crate::app::App, _args: Vec<&str>) {
    app.log_messages.clear();
}

pub const CLEAR: Command = Command {
    execute: &execute_clear,
    name: "clear",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Clear all messages from the log panel.",
};
