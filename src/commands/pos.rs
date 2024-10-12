use rug::{Assign, Float};

use super::Command;

pub fn execute_pos(app: &mut crate::app::App, args: Vec<&str>) {
    // If no args are provided, show the current positino
    if args.is_empty() {
        app.log_info_title(
            "Current Position",
            format!(
                "Real: {:e}\nImag: {:e}\n<green Use the following command to go back to the same position:>\n<command pos {0} {1} >",
                app.render_settings.pos.real(),
                app.render_settings.pos.imag()
            ),
        );
        return
    } 
    // If args were provided, there must be exactly 2 args.
    let real = args[0];
    let imag = args[1];

    if real != "~" {
        let parsed = Float::parse(real);
        if parsed.is_err() { 
            app.log_error("The provided real part must be a valid float or <acc ~>.");
            return
        }
        // We can unwrap because we checked for Err just above
        app.render_settings.pos.mut_real().assign(parsed.unwrap());
        app.redraw_canvas = true;
    }
    if imag != "~" {
        let parsed = Float::parse(imag);
        if parsed.is_err() { 
            app.log_error("The provided imag part must be a valid float or <acc ~>.");
            return
        }
        // We can unwrap because we checked for Err just above
        app.render_settings.pos.mut_imag().assign(parsed.unwrap());
        app.redraw_canvas = true;
    }
}

pub const POS: Command = Command {
    execute: &execute_pos,
    name: "pos",
    accepted_arg_count: &[0, 2],
    basic_desc: "View or set the position of the canvas in the complex plane.",
    detailed_desc: Some(concat!(
        "<green Usage: <command [real] [imag]>>\n",
        "<green Usage: <command [without args]>>\n",
        "If no arguments are given, display the current position. ",
        "Else, position the canvas at the provided coordinates. ",
        "<acc [real]> and <acc [imag]> must be valid floats, ",
        "instead you can also provide <acc ~> to keep the current value.",
        "\n<green Example>: <command pos 0 ~> will set the real part to 0 and keep the imaginary part."
    )),
};
