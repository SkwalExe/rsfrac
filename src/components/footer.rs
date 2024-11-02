//! Contains the `Footer` widget.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};
use tui_markup::compile_with;

use crate::{helpers::markup::get_ratatui_generator, AppState};

/// The footer widget.
pub(crate) struct Footer<'a> {
    state: &'a AppState,
}

impl<'a> Footer<'a> {
    pub(crate) fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut content = String::from(" Actions:");
        let mut line_len = content.len();

        // The maximum length of a footer line.
        let max_line_len = area.width as usize - 2;

        for seg in self.state.footer_text() {
            // If the next segments exceeds the maximum footer line length,
            // Move to the next line.
            if line_len + seg.len() > max_line_len {
                content += "\n";
                line_len = 0;
            }

            content += " ";
            content += seg;

            line_len += seg.len() + 1
        }

        // Highlight the keys
        content = content.replace("[", "[<acc ").replace("]", ">]");

        // Todo: check use of unwrap here
        // Create a colored paragraph from the generated text content.
        let footer = Paragraph::new(compile_with(&content, get_ratatui_generator()).unwrap())
            .block(Block::default().style(Style::default().bg(Color::Rgb(30, 30, 30))));

        footer.render(area, buf);
    }
}
