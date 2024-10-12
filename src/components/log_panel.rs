use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, MouseEvent},
    layout::{Margin, Rect, Size},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_markup::compile_with;
use tui_scrollview::ScrollView;

use crate::{
    app::App,
    helpers::{markup::get_ratatui_generator, Focus},
};

pub struct LogPanel<'a> {
    app: &'a App,
}
impl<'a> LogPanel<'a> {
    pub const FOOTER_TEXT: &'static [&'static str] = &["ScrollUp[k/up]", "ScrollDown[j/down]"];
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
    pub fn handle_mouse_event(app: &mut App, _event: MouseEvent) {
        app.focused = Focus::LogPanel;
    }
    pub fn handle_event(app: &mut App, code: KeyCode) {
        let mut app_state = app.app_state.lock().unwrap();
        match code {
            KeyCode::Up | KeyCode::Char('k') => app_state.log_panel_scroll_state.scroll_up(),
            KeyCode::Down | KeyCode::Char('j') => app_state.log_panel_scroll_state.scroll_down(),
            KeyCode::Left | KeyCode::Char('h') => app_state.log_panel_scroll_state.scroll_left(),
            KeyCode::Right | KeyCode::Char('l') => app_state.log_panel_scroll_state.scroll_right(),
            _ => {}
        }
    }
}

impl<'a> Widget for LogPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block_style = Style::default().fg(if self.app.focused == Focus::LogPanel {
            Color::LightBlue
        } else {
            Color::DarkGray
        });
        let block = Block::new()
            .title("Log Panel")
            .style(block_style)
            .borders(Borders::ALL);
        block.render(area, buf);

        // Get the area inside the block
        let area = area.inner(Margin::new(1, 1));
        // Vec storing each paragraph
        let mut lines = Vec::with_capacity(self.app.log_messages.len());

        // The height of all the paragraphs
        let mut scrollable_height = 0u16;

        let para_width = area.width - 1;

        for message in &self.app.log_messages {
            // Create a ganerator with custom tags
            let gen = get_ratatui_generator();
            // Create a paragraph, using the generator to parse markup syntax
            let para = Paragraph::new(
                compile_with(
                    message.as_str(), gen
                ).unwrap_or(
                    format!(
                        "There was an error parsing the log message, please report this at https://github.com/SkwalExe/rsfrac/issues. \nMessage: {}",
                        message.as_str()
                    ).into()
                )
            )
            .wrap(Wrap { trim: false });

            // Calculate how tall the paragraph will be when fit in the width of the log panel
            let line_count = para.line_count(para_width) as u16;

            // Get the area of the paragraph inside the scrollview
            let para_area = Rect::new(0, scrollable_height, para_width, line_count);
            // +1 -> space for the separator
            scrollable_height += line_count + 1;
            lines.push((para, para_area));
        }
        // width - 1 for the scrollbar
        // height - 1 to ignore the blank space for the next separator
        let size = Size::new(area.width - 1, scrollable_height.saturating_sub(1));

        let mut scroll_view = ScrollView::new(size);

        let mut first_message = true;
        while !lines.is_empty() {
            let line = lines.remove(0);

            if first_message {
                first_message = false
            } else {
                scroll_view.render_widget(
                    Paragraph::new("-".repeat(area.width.into()))
                        .style(Style::default().fg(Color::Rgb(42, 42, 42))),
                    Rect::new(line.1.x, line.1.y - 1, line.1.width, 1),
                );
            }
            scroll_view.render_widget(line.0, line.1);
        }

        let state = &mut self.app.app_state.lock().unwrap().log_panel_scroll_state;
        scroll_view.render(area, buf, state);
    }
}
