use futures::executor;

use super::Command;
use crate::{
    fractals::{get_frac_index_by_name, FRACTALS},
    AppState,
};

pub(crate) fn execute_frac(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        state.log_raw(format!(
            "Current fractal: <acc {}>\nAvailable fractals: {}",
            state.render_settings.get_frac_obj().name,
            FRACTALS
                .iter()
                .map(|f| format!("<acc {}>", f.name))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        return Ok(());
    }

    let info = args[0] == "info";

    if info && args.len() != 2 {
        return Err("Expected the a fractal name after <command info>".to_string());
    }

    if !info && args.len() == 2 {
        return Err(format!(
            "Expected the first of the two arguments to be <command info>, but got <command {}>",
            args[0]
        ));
    }

    let frac_name = args[if info { 1 } else { 0 }];
    let frac_i = get_frac_index_by_name(frac_name).ok_or(format!(
        "Could not find fractal with name: <command {frac_name}>"
    ))?;

    if info {
        let frac_obj = &FRACTALS[frac_i];
        state.log_info_title(frac_obj.name, frac_obj.details);
    } else {
        state.request_redraw();
        state.render_settings.frac_index = frac_i;
        state.log_success(format!("Successfully selected fractal: <acc {frac_name}>."));
        if let Err(err) = executor::block_on(state.render_settings.update_fractal_shader(None)) {
            state.render_settings.use_gpu = false;
            return Err(format!(
                "Disabling GPU mode because fractal shader could not be loaded: {err}"
            ));
        };
    }
    Ok(())
}

pub(crate) const FRAC: Command = Command {
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
