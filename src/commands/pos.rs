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
        return;
    }
    // If args were provided, there must be exactly 2 args.
    let real = args[0];
    let imag = args[1];
    let set_real = real != "~";
    let set_imag = imag != "~";
    let parsed_real;
    let parsed_imag;

    // todo: I don't like this
    parsed_real = Float::parse(real);
    parsed_imag = Float::parse(imag);

    if (set_real && parsed_real.is_err()) || (set_imag && parsed_imag.is_err()) {
        app.log_error("The provided real and imaginary parts must be valid floats or <acc ~>.");
        return;
    }

    if set_real {
        app.render_settings
            .pos
            .mut_real()
            .assign(parsed_real.unwrap());
    }
    if set_imag {
        app.render_settings
            .pos
            .mut_imag()
            .assign(parsed_imag.unwrap());
    }
    app.redraw_canvas = true;
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
