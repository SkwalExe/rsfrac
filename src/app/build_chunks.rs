use ratatui::layout::{Constraint, Direction as Dir, Layout, Margin, Rect};

use crate::app::App;

impl App {
    /// Takes the area of the entire frame and splits it
    /// into sub areas for the different components
    pub fn build_chunks(&mut self, area: Rect) {
        let direction = if area.width <= area.height * 2 {
            Dir::Vertical
        } else {
            Dir::Horizontal
        };

        // Split the body and the footer
        let chunks = Layout::default()
            .direction(Dir::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(2)])
            .split(area);

        self.chunks.body = chunks[0];
        self.chunks.footer = chunks[1];

        // The two main vertical chunks
        let chunks = Layout::default()
            .direction(direction)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(self.chunks.body);

        self.chunks.canvas = chunks[0];
        self.chunks.canvas_inner = chunks[0].inner(Margin::new(1, 1));
        self.chunks.side = chunks[1];

        // The log panel and input chunks
        let side_chunks = Layout::default()
            .direction(Dir::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(self.chunks.side);

        self.chunks.log_panel = side_chunks[0];
        self.chunks.input = side_chunks[1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_chunks_horizontal() {
        let mut app = App::default();
        app.build_chunks(Rect::new(0, 0, 200, 50));
        assert_eq!(app.chunks.footer, Rect::new(0, 48, 200, 2));
        assert_eq!(app.chunks.canvas, Rect::new(0, 0, 140, 48));
        assert_eq!(app.chunks.canvas_inner, Rect::new(1, 1, 138, 46));
        assert_eq!(app.chunks.log_panel, Rect::new(140, 0, 60, 45));
        assert_eq!(app.chunks.input, Rect::new(140, 45, 60, 3));
    }
    #[test]
    fn test_build_chunks_vertical() {
        let mut app = App::default();
        app.build_chunks(Rect::new(0, 0, 100, 100));
        assert_eq!(app.chunks.footer, Rect::new(0, 98, 100, 2));
        assert_eq!(app.chunks.canvas, Rect::new(0, 0, 100, 69));
        assert_eq!(app.chunks.canvas_inner, Rect::new(1, 1, 98, 67));
        assert_eq!(app.chunks.log_panel, Rect::new(0, 69, 100, 26));
        assert_eq!(app.chunks.input, Rect::new(0, 95, 100, 3));
    }
}
