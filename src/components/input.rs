use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent},
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{app::AppState, commands::get_commands, helpers::Focus};

pub(crate) struct Input<'a> {
    app_state: &'a AppState,
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
    pub(crate) const FOOTER_TEXT: &'static [&'static str] = &["Press [Enter] to run the command"];
    pub(crate) fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
    pub(crate) fn handle_mouse_event(app: &mut AppState, _event: MouseEvent) {
        app.focused = Focus::Input;
    }
    pub(crate) fn run_command(app_state: &mut AppState) {
        let input = String::from(app_state.command_input.value());
        app_state.command_input.reset();

        let mut args: Vec<_> = input.split_whitespace().collect();

        // Do nothing more if the command is empty
        if args.is_empty() {
            return;
        }

        // The first argument is the command name
        let command_name = args.remove(0);
        if let Some(command) = get_commands().get(command_name).copied() {
            app_state.log_raw(format!("<command \\> {}>", input));
            if !command.accepted_arg_count.contains(&args.len()) {
                // If the number of provided arguments in not in the accepted argument
                // count list, then print an error and return
                app_state.log_error(
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
            (command.execute)(app_state, args);
        } else {
            // TODO: centralize this message, that is used in other places
            app_state.log_error(format!(
                concat!(
                    "Command not found: <bgred,white {}>. ",
                    "Use <command help> for an overview of available commands."
                ),
                command_name
            ))
        }
    }
    pub(crate) fn handle_event(app_state: &mut AppState, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => Input::run_command(app_state),
            _ => {
                app_state.command_input.handle_event(&Event::Key(key));
            }
        };
    }
}

impl<'a> Widget for Input<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = Style::default().fg(if self.app_state.focused == Focus::Input {
            Color::LightBlue
        } else {
            Color::DarkGray
        });
        let scroll = self
            .app_state
            .command_input
            .visual_scroll(area.width as usize);
        let input = Paragraph::new(self.app_state.command_input.value())
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
