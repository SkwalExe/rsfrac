//! Contains the `Input` widget.

use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent},
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_input::Input as TuiInput;

use crate::{
    commands::get_command_by_name,
    helpers::{markup::esc, Focus},
    AppState,
};
use tui_input::backend::crossterm::EventHandler;

pub(crate) struct Input<'a> {
    state: &'a AppState,
}

fn enumerate_strings(elements: &[usize]) -> String {
    assert!(!elements.is_empty());
    let elements: Vec<_> = elements.iter().map(usize::to_string).collect();
    if elements.len() == 1 {
        return elements[0].clone();
    }
    // We can unwrap because we know the length is >1
    let split = elements.split_last().unwrap();
    split.1.join(", ") + " or " + split.0
}

impl<'a> Input<'a> {
    pub(crate) const FOOTER_TEXT: &'static [&'static str] = &[
        "Execute the command [Enter]",
        "Repeat last command [Ctrl+R]",
        "Edit last commands [Ctrl+E/Arrows]",
    ];
    pub(crate) fn new(state: &'a AppState) -> Self {
        Self { state }
    }
    pub(crate) fn handle_mouse_event(app: &mut AppState, _event: MouseEvent) {
        app.focused = Focus::Input;
    }
    /// Run the last command that was ran.
    pub(crate) fn run_last_command(state: &mut AppState) {
        Input::run_command(state, state.get_command(0));
    }
    /// Runs the command that is currently entered in the command input.
    pub(crate) fn run_current_command(state: &mut AppState) {
        // ignore whitespace around the command
        let input = state.command_input.0.value().trim().to_string();
        state.command_input.0.reset();

        Input::run_command(state, input);
    }
    /// Run the command gived as argument.
    pub(crate) fn run_command(state: &mut AppState, input: String) {
        state.command_input.1 = -1;

        if !input.is_ascii() {
            state.log_error("Your command cannot contain other than ASCII characters, aborting.");
            return;
        }

        // Save the command in history only if it is different from the last command
        if input != state.get_command(0) {
            state.last_commands.insert(0, input.clone());
        }

        let mut args: Vec<_> = input.split_whitespace().collect();

        // Do nothing more if the command is empty
        if args.is_empty() {
            return;
        }

        // The first argument is the command name
        let command_name = args.remove(0);
        if let Some(command) = get_command_by_name(command_name) {
            state.log_raw(format!("<command \\> {}>", esc(&input)));
            if !command.accepted_arg_count.contains(&args.len()) {
                // If the number of provided arguments in not in the accepted argument
                // count list, then print an error and return
                state.log_error(
                    format!(
                        concat!("<bg:black,fg:white {}> expects {} arguments but got {}. ", 
                            "Use <bg:black,fg:white help {}> for more details on the usage of this command."), 
                        command.name,
                        enumerate_strings(command.accepted_arg_count),
                        args.len(),
                        command.name
                    )
                );
                return;
            }

            let res = (command.execute)(state, args);
            state.handle_res(res);
        } else {
            // TODO: centralize this message, that is used in other places
            state.log_error(format!(
                concat!(
                    "Command not found: <bgred,white {}>. ",
                    "Use <command help> for an overview of available commands."
                ),
                esc(command_name)
            ))
        }
    }
    pub(crate) fn handle_event(state: &mut AppState, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => Input::run_current_command(state),
            KeyCode::Down => {
                state.command_input.1 = (state.command_input.1 - 1).max(-1);
                state.command_input.0 = TuiInput::new(state.get_command(state.command_input.1));
            }
            k if {
                (k == KeyCode::Char('e') && key.modifiers == KeyModifiers::CONTROL)
                    || k == KeyCode::Up
            } =>
            {
                state.command_input.1 =
                    (state.command_input.1 + 1).min(state.last_commands.len() as i32 - 1);
                state.command_input.0 = TuiInput::new(state.get_command(state.command_input.1));
            }
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
                Input::run_last_command(state)
            }
            _ => {
                state.command_input.1 = -1;
                state.command_input.0.handle_event(&Event::Key(key));
            }
        };
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = Style::default().fg(if self.state.focused == Focus::Input {
            Color::LightBlue
        } else {
            Color::DarkGray
        });
        let scroll = self
            .state
            .command_input
            .0
            .visual_scroll(area.width as usize);
        let input = Paragraph::new(self.state.command_input.0.value())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(border_style)
                    .title("Command Input"),
            )
            .scroll((0, scroll as u16))
            .style(Style::default().reset());

        input.render(area, buf);
    }
}
