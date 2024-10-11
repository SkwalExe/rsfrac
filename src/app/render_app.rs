use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, Frame};

use crate::{
    components::{canvas::Canvas, footer::Footer, input::Input, log_panel::LogPanel},
    helpers::Focus,
};

use super::App;

impl App {
    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        // The ratatui API is sometimes very annoying
        // This must be added here because .set_cursor_pos is not implemented on Buffers
        if self.focused == Focus::Input {
            let scroll = self
                .command_input
                .visual_scroll(self.chunks.input.width as usize);
            frame.set_cursor_position((
                self.chunks.input.x
                    + 1
                    + (self.command_input.visual_cursor().max(scroll) - scroll) as u16,
                self.chunks.input.y + 1,
            ))
        }
    }
}

impl Widget for &App {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let canvas = Canvas::new(self);
        canvas.render(self.chunks.canvas, buf);

        let log_panel = LogPanel::new(self);
        log_panel.render(self.chunks.log_panel, buf);

        let input = Input::new(self);
        input.render(self.chunks.input, buf);

        let footer = Footer::new(self);
        footer.render(self.chunks.footer, buf);
    }
}
