use super::Command;
use crate::{AppState, VERSION};

pub(crate) fn execute_version(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    state.log_info_title(
        "Rsfrac version",
        format!("Rsfrac is running version <acc {}>", VERSION),
    );
    Ok(())
}

pub(crate) const VERSION_COMMAND: Command = Command {
    execute: &execute_version,
    name: "version",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Display the version number of Rsfrac.",
};
