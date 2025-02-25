use std::sync::Mutex;

use super::Command;
use crate::{helpers::markup::esc, AppState};

pub(crate) fn execute_history(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    // Removes the last command from the history
    state.last_commands.remove(0);

    static LAST_COMMANDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    if args.is_empty() {
        let mut locked = LAST_COMMANDS.lock().unwrap();
        *locked = state.last_commands.clone();

        state.log_info(if locked.is_empty() {
            "No previous commands.".to_string()
        } else {
            format!("Below is shown your command history:\n{}", {
                let mut res = String::new();
                for (i, command) in locked.iter().enumerate() {
                    res += &format!("<acc {i}>: {}\n", esc(command));
                }
                res.trim().to_string()
            })
        });
        return Ok(());
    }

    let index = args[0].to_string();

    // Check if the provided index can be parsed to an integer
    let num = index
        .parse::<usize>()
        .map_err(|_| "The provided index isn't a valid integer.")?;
    let locked = LAST_COMMANDS.lock().unwrap();
    // Check if the last commands can be indexed by this integer
    let command = locked
        .get(num)
        .ok_or("No commands associated with the given index.")?;
    // ugly but it works
    state.command_input.0 = state.command_input.0.clone().with_value(command.clone());

    Ok(())
}

pub(crate) const HISTORY: Command = Command {
    execute: &execute_history,
    name: "history",
    aliases: &["hist"],
    accepted_arg_count: &[0, 1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [no args]>>\n",
        "When no args are provided, display all the ",
        "previous commands and give each of the an index.\n",
        "<green Usage: <command [integer]>>\n",
        "Edit the command at the specified index. ",
        "Must be ran after <command history> without arguments.\n",
        "It is also possible to navigate the command history using the up and down arrow keys.\n",
    )),
    basic_desc: "Displays and navigates the command history.",
};
