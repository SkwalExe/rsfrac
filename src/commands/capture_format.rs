use image::ImageFormat;

use super::Command;
use crate::AppState;

pub(crate) fn execute_capture_format(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        state.log_info(format!(
            "Selected capture format: <acc {}>\nAvailable formats:\n{}.",
            state.render_settings.image_format.extensions_str()[0],
            ImageFormat::all()
                .map(|f| format!("<acc {}>", f.extensions_str()[0]))
                .collect::<Vec<String>>()
                .join(", ")
        ))
    } else {
        let ext = args[0];
        state.render_settings.image_format =
            ImageFormat::from_extension(ext).ok_or("Image format not recognized.")?;
        state.log_success(format!(
            "Successfully selected capture format: <acc {}>",
            state.render_settings.image_format.extensions_str()[0]
        ));
    }
    Ok(())
}
pub(crate) const CAPTURE_FORMAT: Command = Command {
    execute: &execute_capture_format,
    name: "capture_format",
    aliases: &["cf"],
    accepted_arg_count: &[0, 1],

    detailed_desc: Some(concat!(
        "<green Usage: <command [no args]>>\n",
        "Display the available file formats.\n",
        "<green Usage: <command [file extension]>>\n",
        "Select one of the available image formats."
    )),
    basic_desc:
        "Change the image format used to capture screenshots using the <command capture> command.",
};
