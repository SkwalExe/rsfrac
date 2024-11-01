use crate::{app::AppState, app::VERSION};

use super::Command;

pub(crate) fn execute_version(app_state: &mut AppState, _args: Vec<&str>) {
    app_state.log_info_title(
        "Rsfrac version",
        format!("Rsfrac is running version <acc {}>", VERSION),
    )
}

pub(crate) const VERSION_COMMAND: Command = Command {
    execute: &execute_version,
    name: "version",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Display the version number of Rsfrac.",
};
