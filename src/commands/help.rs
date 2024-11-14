use super::{get_commands_list, get_commands_map, Command};
use crate::AppState;

pub(crate) fn execute_help(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        state.log_info_title(
            "Available commands",
            format!(
                concat!(
                    "<acc {}>\n<green use <command help +> to get a list ",
                    "of all the commands and a basic description. ",
                    "Use <command help command_name> to get more info about a command.>"
                ),
                get_commands_list().map(|command| command.name).join(", "),
            ),
        );
        return Ok(());
    }

    if args[0] == "+" {
        state.log_info_title(
            "Available commands",
            format!(
                "{}\n<green Use <command help command_name> to get more info about a command.>",
                get_commands_list()
                    .map(|c| format!("- <acc {}>: {}", c.name, c.basic_desc))
                    .join("\n")
            ),
        );

        return Ok(());
    }

    let command_name = args[0];
    let command = get_commands_map().remove(command_name).ok_or(format!(
        concat!(
            "Command not found: <bgred,white {}>. ",
            "Use <command help> for an overview of available commands."
        ),
        command_name
    ))?;

    state.log_info_title(
        command.name,
        format!(
            "{}\n{}",
            command.basic_desc,
            command.detailed_desc.unwrap_or_default()
        ),
    );
    Ok(())
}
pub(crate) const HELP: Command = Command {
    execute: &execute_help,
    name: "help",
    accepted_arg_count: &[0, 1],
    basic_desc: "Provide information about the available commands.",
    detailed_desc: None,
};
