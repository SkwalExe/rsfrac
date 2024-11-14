//! Contains logging logic.

use ansi_term::ANSIStrings;
use ratatui::DefaultTerminal;
use tui_markup::compile_with;

use crate::helpers::markup::get_ansi_generator;
use crate::{App, AppState};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

const LOG_MESSAGE_LIMIT: usize = 500;

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

impl AppState {
    /// Print the initial log messages
    pub(crate) fn initial_message(&mut self) {
        self.log_raw(format!(
            concat!(
                "<bgacc Welcome to Rsfrac v{}>\n",
                "Author: <acc Léopold Koprivnik>\n",
                "Github Repo: <acc SkwalExe/rsfrac>",
            ),
            VERSION
        ));
        self.log_raw(concat!(
            "If you are experiencing slow rendering, ",
            "try to reduce the size of your terminal.",
        ));
        self.log_raw(concat!(
            "You can switch between the canvas, the log panel and ",
            "the command input using <acc tab>. ",
            "Use the <acc help> command for more information."
        ));
    }

    /// Creates a new log message.
    pub(crate) fn log_raw(&mut self, message: impl Into<String>) {
        self.log_messages.push(message.into());
        if self.log_messages.len() > LOG_MESSAGE_LIMIT {
            self.log_messages.remove(0);
        }
        let state = &mut self.log_panel_scroll_state.lock().unwrap();
        state.scroll_to_bottom();
    }

    /// Creates a new success log message with the provided message and a fixed title.
    pub(crate) fn log_success(&mut self, message: impl Into<String>) {
        self.log_success_title("Success", message.into())
    }
    /// Creates a new info log message with the provided message and a fixed title.
    pub(crate) fn log_info(&mut self, message: impl Into<String>) {
        self.log_info_title("Info", message.into())
    }
    /// Creates a new error log message with the provided message and a fixed title.
    pub(crate) fn log_error(&mut self, message: impl Into<String>) {
        self.log_error_title("Error", message);
    }

    /// Creates a new success log message with the provided title and message.
    pub(crate) fn log_success_title(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.log_raw(format!("<bggreen  {} >\n{}", title.into(), message.into()))
    }
    /// Creates a new info log message with the provided title and message.
    pub(crate) fn log_info_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!("<bgacc  {} >\n{}", title.into(), message.into()))
    }
    /// Creates a new error log message with the provided title and message.
    pub(crate) fn log_error_title(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.log_raw(format!("<bgred  {} >\n{}", title.into(), message.into()))
    }
}
