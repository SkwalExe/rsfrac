use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;

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
        for seg in self.app.footer_text() {
            if content.len() + seg.len() > area.width as usize - 2 {
                content += "\n"
            }
            content += " ";
            content += seg;
        }
        let input = Paragraph::new(content)
            .block(Block::default().style(Style::default().bg(Color::Rgb(30, 30, 30))));

        input.render(area, buf);
    }
}
