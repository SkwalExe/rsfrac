use super::Command;
use crate::AppState;

pub(crate) fn execute_quit(state: &mut AppState, _args: Vec<&str>) {
    state.quit = true;
}
pub(crate) const QUIT: Command = Command {
    execute: &execute_quit,
    name: "quit",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Exit from the state and return to your shell session.",
};
