use super::{get_command_by_name, get_commands_list, Command};
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
                    .map(|c| format!(
                        "- <acc {}>{}: {}",
                        c.name,
                        if c.aliases.is_empty() {
                            "".to_string()
                        } else {
                            format!(
                                " ({})",
                                c.aliases
                                    .iter()
                                    .map(|c| format!("<acc {c}>"))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            )
                        },
                        c.basic_desc
                    ))
                    .join("\n")
            ),
        );

        return Ok(());
    }

    let command_name = args[0];
    let command = get_command_by_name(command_name).ok_or(format!(
        concat!(
            "Command not found: <bgred,white {}>. ",
            "Use <command help> for an overview of available commands."
        ),
        command_name
    ))?;

    state.log_info_title(
        command.name,
        format!(
            "Aliases: {}\n{}\n{}",
            command
                .aliases
                .iter()
                .map(|name| format!("<acc {name}>"))
                .collect::<Vec<String>>()
                .join(", "),
            command.basic_desc,
            command.detailed_desc.unwrap_or_default()
        ),
    );
    Ok(())
}
pub(crate) const HELP: Command = Command {
    execute: &execute_help,
    name: "help",
    aliases: &["h"],
    accepted_arg_count: &[0, 1],
    basic_desc: "Provide information about the available commands.",
    detailed_desc: None,
};
