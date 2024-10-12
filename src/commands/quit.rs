use super::Command;

pub fn execute_quit(app: &mut crate::app::App, _args: Vec<&str>) {
    app.quit = true;
}
pub const QUIT: Command = Command {
    execute: &execute_quit,
    name: "quit",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Exit from the app and return to your shell session."
};
