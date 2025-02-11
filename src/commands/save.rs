use chrono::Local;

use super::Command;
use crate::{helpers::markup::esc, AppState};

pub(crate) const SAVE_EXTENSION: &str = ".rsf";

pub(crate) fn execute_save(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let filename = if args.is_empty() {
        format!(
            "{} {}",
            state.render_settings.get_frac_obj().name,
            Local::now().format("%F %H-%M-%S"),
        )
    } else {
        args[0].to_string()
    } + SAVE_EXTENSION;

    state.render_settings.save(&filename)?;
    state.log_success(format!("State successfully saved as <command {}>.", esc(filename)));
    Ok(())
}

pub(crate) const SAVE: Command = Command {
    execute: &execute_save,
    name: "save",
    aliases: &[],
    accepted_arg_count: &[0, 1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [file path]>>\n",
        "Save the current state using the provided file path.\n",
        "<green Usage: <command [no args]>>\n",
        "Save the current state using a generic name.\n",
    )),
    basic_desc: "Save the current application state to a file that can be loaded back later with the <command load> command.",
};
