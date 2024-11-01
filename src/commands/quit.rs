use crate::app::AppState;

use super::Command;

pub(crate) fn execute_quit(app_state: &mut AppState, _args: Vec<&str>) {
    app_state.quit = true;
}
pub(crate) const QUIT: Command = Command {
    execute: &execute_quit,
    name: "quit",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Exit from the app_state and return to your shell session.",
};
