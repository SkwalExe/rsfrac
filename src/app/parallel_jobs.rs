use chrono::Utc;
use image::ImageBuffer;
use ratatui::style::Color;

use crate::{app_state::DivergMatrix, helpers::Vec2, AppState};

const LINES_PER_PASS: i32 = 64;

#[derive(Clone)]
pub(crate) struct Screenshot {
    size: Vec2<i32>,
    current_line: i32,
    diverg_matrix: DivergMatrix,
}
impl Screenshot {
    pub(crate) fn new(size: Vec2<i32>) -> Self {
        Self {
            size,
            current_line: 0,
            diverg_matrix: Default::default(),
        }
    }
}
impl Screenshot {
    pub(crate) fn run(&mut self, state: &mut AppState) -> bool {
        let _current_line = self.size.y.min(self.current_line + LINES_PER_PASS);

        self.diverg_matrix.append(&mut state.get_diverg_lines(
            &self.size,
            self.current_line,
            _current_line - 1,
        ));

        if _current_line >= self.size.y - 1 {
            let buf = ImageBuffer::from_fn(self.size.x as u32, self.size.y as u32, |x, y| {
                let color = state.color_from_div(&self.diverg_matrix[y as usize][x as usize]);
                if let Color::Rgb(r, g, b) = color {
                    image::Rgb([r, g, b])
                } else {
                    image::Rgb([0, 0, 0])
                }
            });
            let filename = format!(
                "{}{}.png",
                state.render_settings.get_frac_obj().name,
                Utc::now().timestamp()
            );

            let _ = buf.save_with_format(&filename, image::ImageFormat::Png);

            state.log_success(format!(
                "Screenshot ({}x{}) saved to <acc {}>",
                self.size.x, self.size.y, filename
            ));

            return true;
        }
        self.current_line = _current_line;
        state.log_info(format!(
            "Screenshot progression: [<acc {}%>]",
            self.current_line * 100 / self.size.y
        ));
        false
    }
}
