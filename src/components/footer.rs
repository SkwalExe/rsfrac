use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};
use tui_markup::compile_with;

use crate::{app::App, helpers::markup::get_ratatui_generator};

pub struct Footer<'a> {
    app: &'a App,
}

impl<'a> Footer<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut content = String::from(" Actions:");
        let mut line_len = content.len();
        for seg in self.app.footer_text() {
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
