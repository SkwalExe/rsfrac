use crate::fractals::FRACTALS;

use super::Command;

pub fn execute_frac(app: &mut crate::app::App, args: Vec<&str>) {
    if args.is_empty() {
        app.log_raw(format!(
            "Current fractal: <acc {}>\nAvailable fractals: {}",
            app.render_settings.get_frac_obj().name,
            FRACTALS
                .iter()
                .map(|f| format!("<acc {}>", f.name))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        return;
    }

    let info = args[0] == "info";

    if info && args.len() != 2 {
        app.log_error("Expected the a fractal name after <command info>");
        return;
    }

    if !info && args.len() == 2 {
        app.log_error(format!(
            "Expected the first of the two arguments to be <command info>, but got <command {}>",
            args[0]
        ));
        return;
    }

    let frac_name = args[if info { 1 } else { 0 }];
    let frac_i = app.render_settings.get_frac_index_by_name(frac_name);
    match frac_i {
        None => {
            app.log_error(format!(
                "Could not find fractal with name: <command {frac_name}>"
            ));
        }
        Some(frac_i) => {
            if info {
                let frac_obj = &FRACTALS[frac_i];
                app.log_info_title(frac_obj.name, frac_obj.details);
            } else {
                app.render_settings.frac_index = frac_i;
                app.log_success(format!("Successfully selected fractal: <acc {frac_name}>."));
            }
        }
    }
}

pub const FRAC: Command = Command {
    execute: &execute_frac,
    name: "frac",
    accepted_arg_count: &[0, 1, 2],
    detailed_desc: Some(concat!(
        "<green Usage: <command [frac]>>\n",
        "Select the provided fractal.\n",
        "<green Usage: <command info [frac]>>\n",
        "Give info about the specified fractal.\n",
        "<green Usage: <command [without args]>>\n",
        "List the available fractals.",
    )),
    basic_desc: "List available fractals, and provide info about them.",
};
