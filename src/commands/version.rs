use crate::app::logging::VERSION;

use super::Command;

pub fn execute_version(app: &mut crate::app::App, _args: Vec<&str>) {
    app.log_info_title("Rsfrac version", format!("Rsfrac is running version <acc {}>", VERSION))
}

pub const VERSION_COMMAND: Command = Command {
    execute: &execute_version,
    name: "version",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Display the version number of Rsfrac."
};
