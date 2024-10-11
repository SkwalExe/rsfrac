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
        let input = Paragraph::new(format!(" Actions: {}", self.app.footer_text().join(" ")))
            .block(Block::default().style(Style::default().bg(Color::Rgb(70, 70, 70))));

        input.render(area, buf);
    }
}
