use std::str::FromStr;

use super::Command;
use crate::{
    helpers::{markup::esc, SavedState},
    AppState,
};

pub(crate) fn execute_load_remote(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let url = args[0];
    let res = reqwest::blocking::get(url)
        .map_err(|err| esc(format!("Could not perform HTTP/S request: {}", esc(err))))?
        .text()
        .map_err(|err| esc(format!("Could not read HTTP/S response: {}", esc(err))))?;

    let saved: SavedState = SavedState::from_str(&res)?;
    state.apply(saved, url);
    Ok(())
}

pub(crate) const LOAD_REMOTE: Command = Command {
    execute: &execute_load_remote,
    name: "load_remote",
    aliases: &["lr"],
    accepted_arg_count: &[1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [url]>>\n",
        "Load the state file accessible at the specified URL. ",
    )),
    basic_desc: concat!(
        "Load a state file from a URL.",
        "State files are created with the <command save> command."
    ),
};
