use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent},
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{app::App, commands::get_command, helpers::Focus};

pub struct Input<'a> {
    app: &'a App,
}

impl<'a> Input<'a> {
    pub const FOOTER_TEXT: &'static [&'static str] = &["Press [Enter] to run the command"];
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
    pub fn handle_mouse_event(app: &mut App, _event: MouseEvent) {
        app.focused = Focus::Input;
    }
    pub fn handle_event(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let input = String::from(app.command_input.value());
                app.command_input.reset();

                let mut args: Vec<_> = input.split_whitespace().collect();

                // Do nothing more if the command is empty
                if args.is_empty() {
                    return;
                }

                // The first argument is the command name
                let command_name = args.remove(0);
                let command = get_command(command_name);
                if let Some(command) = command {
                    command(app, args);
                } else {
                    app.log_error(format!("Command not found: <bgred,white {command_name}>"))
                }
            }
            _ => {
                app.command_input.handle_event(&Event::Key(key));
            }
        };
    }
}

impl<'a> Widget for Input<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = Style::default().fg(if self.app.focused == Focus::Input {
            Color::LightBlue
        } else {
            Color::DarkGray
        });
        let scroll = self.app.command_input.visual_scroll(area.width as usize);
        let input = Paragraph::new(self.app.command_input.value())
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
