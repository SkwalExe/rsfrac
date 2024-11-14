use super::Command;
use crate::AppState;

pub(crate) fn execute_clear(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    state.log_messages.clear();
    Ok(())
}

pub(crate) const CLEAR: Command = Command {
    execute: &execute_clear,
    name: "clear",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Clear all messages from the log panel.",
};
