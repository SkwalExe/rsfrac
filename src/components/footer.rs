use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};
use tui_markup::compile_with;

use crate::{app::AppState, helpers::markup::get_ratatui_generator};

pub(crate) struct Footer<'a> {
    app_state: &'a AppState,
}

impl<'a> Footer<'a> {
    pub(crate) fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut content = String::from(" Actions:");
        let mut line_len = content.len();
        for seg in self.app_state.footer_text() {
            if line_len + seg.len() > area.width as usize - 2 {
                content += "\n";
                line_len = 0;
            }
            content += " ";
            content += seg;
            line_len += seg.len() + 1
        }

        // Highlight the keys
        let parsed = content.replace("[", "[<acc ").replace("]", ">]");

        // Todo: check use of unwrap here
        let input = Paragraph::new(compile_with(&parsed, get_ratatui_generator()).unwrap())
            .block(Block::default().style(Style::default().bg(Color::Rgb(30, 30, 30))));

        input.render(area, buf);
    }
}
