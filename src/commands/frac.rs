use super::Command;
use crate::{
    fractals::{get_frac_index_by_name, FRACTALS}, helpers::markup::esc, AppState
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
        "Could not find fractal with name: <command {}>", esc(frac_name)
    ))?;

    if info {
        let frac_obj = &FRACTALS[frac_i];
        state.log_info_title(frac_obj.name, frac_obj.details);
    } else {
        state.request_redraw();
        state.render_settings.select_fractal(frac_i)?;
        state.log_success(format!(
            "Successfully selected fractal: <acc {}>.",
            state.render_settings.get_frac_obj().name
        ));
    }
    Ok(())
}

pub(crate) const FRAC: Command = Command {
    execute: &execute_frac,
    name: "frac",
    aliases: &["f"],
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

#[cfg(test)]
mod tests {
    use crate::AppState;

    use super::execute_frac;

    #[test]
    fn test_frac_command() {
        let mut state = AppState::default();
        // `frac ju` should return Ok and the Julia fracal should be selected
        execute_frac(&mut state, vec!["ju"]).unwrap();
        assert_eq!(state.render_settings.get_frac_obj().name, "Julia");

        // `frac non_exist` should return Err
        assert!(execute_frac(&mut state, vec!["non_exist"]).is_err());

        // `frac info mandel` should return Ok and a log message should have been added
        let count_before = state.log_messages.len();
        execute_frac(&mut state, vec!["info", "mandel"]).unwrap();
        assert_eq!(state.log_messages.len(), count_before + 1);
        //
        // `frac` should return Ok and a log message should have been added
        let count_before = state.log_messages.len();
        execute_frac(&mut state, vec![]).unwrap();
        assert_eq!(state.log_messages.len(), count_before + 1);

        // `frac info non_exist` should return Err
        assert!(execute_frac(&mut state, vec!["info", "non_exist"]).is_err());
    }
}
