use crate::app::AppState;

use super::{get_commands, Command};

pub(crate) fn execute_help(app_state: &mut AppState, args: Vec<&str>) {
    if args.is_empty() {
        app_state.log_info_title(
            "Available commands",
            format!(
                concat!(
                    "<acc {}>\n<green use <command help +> to get a list ",
                    "of all the commands and a basic description. ",
                    "Use <command help command_name> to get more info about a command.>"
                ),
                get_commands()
                    .iter()
                    .map(|c| c.1.name)
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        );
        return;
    }

    if args[0] == "+" {
        app_state.log_info_title(
            "Available commands",
            format!(
                "{}\n<green Use <command help command_name> to get more info about a command.>",
                get_commands()
                    .iter()
                    .map(|c| format!("- <acc {}>: {}", c.1.name, c.1.basic_desc))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        );

        return;
    }

    let command_name = args[0];
    if let Some(command) = get_commands().get(command_name) {
        app_state.log_info_title(
            command.name,
            format!(
                "{}\n{}",
                command.basic_desc,
                command.detailed_desc.unwrap_or_default()
            ),
        );
    } else {
        app_state.log_error(format!(
            concat!(
                "Command not found: <bgred,white {}>. ",
                "Use <command help> for an overview of available commands."
            ),
            command_name
        ))
    }
}
pub(crate) const HELP: Command = Command {
    execute: &execute_help,
    name: "help",
    accepted_arg_count: &[0, 1],
    basic_desc: "Provide information about the available commands.",
    detailed_desc: None,
};
