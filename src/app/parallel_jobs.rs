use super::AppState;

pub(crate) trait ParallelJob {
    fn run(&self, app: &mut AppState) -> bool;
}

#[derive(Clone)]
pub(crate) struct Screenshot {}
impl Screenshot {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl ParallelJob for Screenshot {
    fn run(&self, state: &mut AppState) -> bool {
        // let diverg_matrix = app.get_diverg_matrix(Vec2::new(CAP_W as i32, CAP_H as i32));
        // let buf = ImageBuffer::from_fn(CAP_W, CAP_H, |x, y| {
        //     let color = app.color_from_div(&diverg_matrix[y as usize][x as usize]);
        //     if let RatatuiColor::Rgb(r, g, b) = color {
        //         image::Rgb([r, g, b])
        //     } else {
        //         image::Rgb([0, 0, 0])
        //     }
        // });
        // let _ = buf.save_with_format("a.png", image::ImageFormat::Png);

        // for now the screenshot jobs just return true
        // to indicate that it has finished, without doing anything
        state.log_error("The capture command is under development and for now not available.");
        true
    }
}
