use ansi_term::ANSIStrings;
use ratatui::DefaultTerminal;
use tui_markup::compile_with;

use crate::helpers::markup::get_ansi_generator;

use super::App;

const VERSION: &str = env!("CARGO_PKG_VERSION");
impl App {
    pub fn log_raw(&mut self, message: impl Into<String>) {
        self.log_messages.push(message.into());
        let state = &mut self.app_state.lock().unwrap();
        state.log_panel_scroll_state.scroll_to_bottom();
    }

    pub fn log_success_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!("<bggreen  {} >\n{}", title.into(), message.into()))
    }
    pub fn log_success(&mut self, message: impl Into<String>) {
        self.log_success_title("Success", message.into())
    }
    pub fn log_info_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!("<bgacc  {} >\n{}", title.into(), message.into()))
    }
    pub fn log_info(&mut self, message: impl Into<String>) {
        self.log_info_title("Info", message.into())
    }

    pub fn log_error(&mut self, message: impl Into<String>) {
        self.log_error_title("Error", message);
    }
    pub fn log_error_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!(
            "<bgred  {} >\n<red {}>",
            title.into(),
            message.into()
        ))
    }

    /// Print the initial log messages
    pub fn initial_message(&mut self) {
        self.log_raw(format!(
            "<bgacc Welcome to Rsfrac v{VERSION}>\nAuthor: <acc Léopold Koprivnik>\nGithub Repo: <acc SkwalExe/rsfrac>",
        ));
        self.log_raw(
            "If you are experiencing slow rendering, try to reduce the size of your terminal.",
        );
        self.log_raw("You can switch between the canvas, the log panel and the command input using <acc tab>. Use the <acc help> command for more information.");
    }

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

        for message in &self.log_messages {
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
