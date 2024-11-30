//! Contains the `LogPanel` widget.

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
    helpers::{markup::get_ratatui_generator, Focus},
    AppState,
};

pub(crate) struct LogPanel<'a> {
    state: &'a AppState,
}

impl<'a> LogPanel<'a> {
    pub(crate) const FOOTER_TEXT: &'static [&'static str] =
        &["ScrollUp[k/up]", "ScrollDown[j/down]"];
    pub(crate) fn new(state: &'a AppState) -> Self {
        Self { state }
    }
    pub(crate) fn handle_mouse_event(app: &mut AppState, _event: MouseEvent) {
        app.focused = Focus::LogPanel;
    }
    pub(crate) fn handle_event(state: &mut AppState, code: KeyCode) {
        let mut scroll_state = state.log_panel_scroll_state.lock().unwrap();
        match code {
            KeyCode::Up | KeyCode::Char('k') => scroll_state.scroll_up(),
            KeyCode::Down | KeyCode::Char('j') => scroll_state.scroll_down(),
            KeyCode::Left | KeyCode::Char('h') => scroll_state.scroll_left(),
            KeyCode::Right | KeyCode::Char('l') => scroll_state.scroll_right(),
            KeyCode::End => scroll_state.scroll_to_bottom(),
            KeyCode::Home => scroll_state.scroll_to_top(),
            KeyCode::PageUp => scroll_state.scroll_page_up(),
            KeyCode::PageDown => scroll_state.scroll_page_down(),
            _ => {}
        }
    }
}

impl<'a> Widget for LogPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block_style = Style::default().fg(if self.state.focused == Focus::LogPanel {
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
        let mut lines: Vec<(Paragraph, Rect)> = Vec::with_capacity(self.state.log_messages.len());

        // -2 -> Space for the scrollbar on the right and some padding
        let para_width = area.width - 2;

        // the maximum number of lines not to exceed to keep the total surface < u16::MAX.
        let maximum_height = u16::MAX / para_width;

        // The height of all the paragraphs
        let mut scrollable_height = 0u16;

        for message in self
            .state
            .log_messages
            .iter()
            .chain(self.state.prioritized_log_messages.values())
        {
            // Create a ganerator with custom tags
            let gen = get_ratatui_generator();
            // Create a paragraph, using the generator to parse markup syntax
            let para = Paragraph::new(
                compile_with(
                    message.as_str(), gen
                ).unwrap_or(
                    // Display an error message if the markup could not be parsed, which should
                    // never happen
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
            let mut para_area = Rect::new(0, scrollable_height, para_width, line_count);

            // +1 -> space for the separator
            scrollable_height += line_count + 1;

            // Check if adding the next paragraph will make the log panel surface exceed the maximum
            // for u16. Remove elements until the next paragraph fits.
            while scrollable_height >= maximum_height {
                let removed_height = lines.remove(0).1.height + 1;
                scrollable_height -= removed_height;
                para_area.y -= removed_height;

                for para in lines.iter_mut() {
                    para.1.y -= removed_height
                }
            }

            lines.push((para, para_area));
        }

        // height + 5 to give some space at the bottom in case a prioritized
        // message changes rapidly of length
        let size = Size::new(para_width, scrollable_height + 5);

        // ATTENTION
        // After countless hours of debugging, I realized that when
        // the total surface passed here (size) exceeds u16::MAX,
        // the area will be shrunk arbitrarily.
        let mut scroll_view = ScrollView::new(size);

        let mut first_message = true;
        while !lines.is_empty() {
            let line = lines.remove(0);

            if first_message {
                first_message = false
            } else {
                scroll_view.render_widget(
                    Paragraph::new("-".repeat(para_width as usize))
                        .style(Style::default().fg(Color::Rgb(42, 42, 42))),
                    Rect::new(line.1.x, line.1.y - 1, line.1.width, 1),
                );
            }
            scroll_view.render_widget(line.0, line.1);
        }

        let state = &mut self.state.log_panel_scroll_state.lock().unwrap();
        scroll_view.render(area, buf, state);
    }
}
