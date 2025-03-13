use super::Command;
use crate::AppState;

pub(crate) fn execute_quit(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    state.quit = true;
    Ok(())
}
pub(crate) const QUIT: Command = Command {
    execute: &execute_quit,
    name: "quit",
    aliases: &["q", "exit", "leave"],
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Exit from the state and return to your shell session.",
};
