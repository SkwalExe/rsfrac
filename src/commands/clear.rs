use crate::app::AppState;

use super::Command;
pub(crate) fn execute_clear(app_state: &mut AppState, _args: Vec<&str>) {
    app_state.log_messages.clear();
}

pub(crate) const CLEAR: Command = Command {
    execute: &execute_clear,
    name: "clear",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Clear all messages from the log panel.",
};
