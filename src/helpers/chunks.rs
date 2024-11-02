//! Contains the Chunks struct and associated methods.

use ratatui::layout::{Constraint, Direction as Dir, Layout, Margin, Rect};

/// Used to group the (Rect)s corresponding to each
/// app component.
#[derive(Default)]
pub(crate) struct Chunks {
    /// The area of the entire canvas.
    /// Contains the borders and the raster data.
    pub(crate) canvas: Rect,
    /// The area of the sidebar on the right.
    /// Contains the log panel and the command input.
    pub(crate) side: Rect,
    /// The area of the log panel, including the borders.
    pub(crate) log_panel: Rect,
    /// The area of the command input.
    pub(crate) input: Rect,
    /// The area of the footer, which corresponds to the
    /// two lines at the bottom of the screen.
    pub(crate) footer: Rect,
    /// The area of the application body, which is the whole
    /// area above the footer.
    pub(crate) body: Rect,
}

impl Chunks {
    /// The area inside the canvas, ignoring the canvas borders.
    /// Contains only the raster data.
    pub(crate) fn canvas_inner(&self) -> Rect {
        self.canvas.inner(Margin::new(1, 1))
    }

    /// The area inside the log panel, ignoring the borders.
    /// Contains the log messages and the scrollbar.
    pub(crate) fn log_panel_inner(&self) -> Rect {
        self.log_panel.inner(Margin::new(1, 1))
    }
}

impl From<Rect> for Chunks {
    /// Builds a Chunk group by splitting the given area.
    fn from(area: Rect) -> Self {
        // Split the layout differently depending on whether the
        // available space is longer or larger.
        let direction = if area.width <= area.height * 2 {
            Dir::Vertical
        } else {
            Dir::Horizontal
        };

        // In the base area, split the body from the footer.
        let chunks = Layout::default()
            .direction(Dir::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(2)])
            .split(area);
        let body = chunks[0];
        let footer = chunks[1];

        // In the body, split the canvas for the side chunk,
        // horizontally or vertically depending on (direction).
        let chunks = Layout::default()
            .direction(direction)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(body);
        let canvas = chunks[0];
        let side = chunks[1];

        // In the side chunk, split the log panel and the command input.
        let side_chunks = Layout::default()
            .direction(Dir::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(side);
        let log_panel = side_chunks[0];
        let input = side_chunks[1];

        Self {
            body,
            footer,
            canvas,
            side,
            log_panel,
            input,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_chunks_horizontal() {
        let chunks = Chunks::from(Rect::new(0, 0, 200, 50));
        assert_eq!(chunks.footer, Rect::new(0, 48, 200, 2));
        assert_eq!(chunks.canvas, Rect::new(0, 0, 140, 48));
        assert_eq!(chunks.canvas_inner(), Rect::new(1, 1, 138, 46));
        assert_eq!(chunks.log_panel, Rect::new(140, 0, 60, 45));
        assert_eq!(chunks.input, Rect::new(140, 45, 60, 3));
    }
    #[test]
    fn test_build_chunks_vertical() {
        let chunks = Chunks::from(Rect::new(0, 0, 100, 100));
        assert_eq!(chunks.footer, Rect::new(0, 98, 100, 2));
        assert_eq!(chunks.canvas, Rect::new(0, 0, 100, 69));
        assert_eq!(chunks.canvas_inner(), Rect::new(1, 1, 98, 67));
        assert_eq!(chunks.log_panel, Rect::new(0, 69, 100, 26));
        assert_eq!(chunks.input, Rect::new(0, 95, 100, 3));
    }
}
