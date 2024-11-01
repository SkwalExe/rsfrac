use ansi_term::ANSIStrings;
use ratatui::DefaultTerminal;
use tui_markup::compile_with;

use crate::helpers::markup::get_ansi_generator;

use super::App;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
impl App {
    /// Prints the history of log messages before exiting.
    /// Supports the same formatting as the log panel.
    pub fn print_logs(&self, term: &DefaultTerminal) {
        // Add some space between the terminal prompt and the output
        println!();

        // Ruler to separate each log message, like in the log panel
        let rule = "-".repeat(term.size().unwrap().width.into());
        let rule = format!("<dim {rule}>");
        let rule = compile_with(&rule, get_ansi_generator()).unwrap();
        let rule = ANSIStrings(&rule);

        // Do not print the ruler before the first message
        let mut first = true;

        for message in &self.app_state.log_messages {
            // I want ternary operator, this is shit
            if first {
                first = false;
            } else {
                println!("{rule}");
            }

            // I dont like this
            let formatted = &compile_with(message, get_ansi_generator()).unwrap();
            println!("{}", ANSIStrings(formatted));
        }
    }
}
