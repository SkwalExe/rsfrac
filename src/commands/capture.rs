use image::ImageBuffer;
use ratatui::style::Color as RatatuiColor;

use crate::helpers::Vec2;

use super::Command;
const CAP_H: u32 = 200;
const CAP_W: u32 = 400;

pub fn execute_capture(app: &mut crate::app::App, _args: Vec<&str>) {
    let diverg_matrix = app.get_diverg_matrix(Vec2::new(CAP_W as i32, CAP_H as i32));
    let buf = ImageBuffer::from_fn(CAP_W, CAP_H, |x, y| {
        let color = app.color_from_div(&diverg_matrix[y as usize][x as usize]);
        if let RatatuiColor::Rgb(r, g, b) = color {
            image::Rgb([r, g, b])
        } else {
            image::Rgb([0, 0, 0])
        }
    });
    let _ = buf.save_with_format("a.png", image::ImageFormat::Png);
}
pub const CAPTURE: Command = Command {
    execute: &execute_capture,
    name: "capture",
    accepted_arg_count: &[0],

    detailed_desc: Some(concat!(
        "TODO",
    )),
    basic_desc:
        "TODO",
};
