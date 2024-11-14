use std::{fs::File, io::Write};

use chrono::Local;

use super::Command;
use crate::{helpers::SavedState, AppState};

pub(crate) fn execute_save(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let filename = if args.is_empty() {
        format!(
            "{} {}.rsf",
            Local::now().format("%F %H-%M-%S"),
            state.render_settings.get_frac_obj().name
        )
    } else {
        args[0].to_string() + ".rsf"
    };

    let saved_state = SavedState::from(&*state);
    let str = toml::to_string_pretty(&saved_state)
        .map_err(|err| format!("Could not save the current state: {err}"))?;

    let mut file = File::create(&filename)
        .map_err(|err| format!("Could not create <command {filename}>: {err}"))?;

    file.write(str.as_bytes())
        .map_err(|err| format!("Could not write file: {err}"))?;
    state.log_success(format!("State successfully saved as <command {filename}>."));
    Ok(())
}

pub(crate) const SAVE: Command = Command {
    execute: &execute_save,
    name: "save",
    accepted_arg_count: &[0, 1],
    detailed_desc: None,
    basic_desc: "Save the current application state to a file that can be loaded back later with the <command load> command.",
};
