use super::Command;
use crate::{
    app_state::ClickMode,
    AppState,
};

pub(crate) fn execute_click_mode(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        state.log_raw(format!(
            "Current click configuration:\nLeft: <acc {}>\nMiddle: <acc {}>\nRight: <acc {}>\nAvailable click actions:\n{}",
            state.click_config.left, 
            state.click_config.middle, 
            state.click_config.right, 
            ClickMode::all().iter().map(|x| format!("<acc {x}>")).collect::<Vec<String>>().join(", ")
        ));
        return Ok(());
    }

    let button_str = args[0].to_lowercase();

    let action = ClickMode::from(args[1]).ok_or("The given action was not recognized")?;

    *match button_str.as_str() {
        "left" => &mut state.click_config.left,
        "middle" => &mut state.click_config.middle,
        "right" => &mut state.click_config.right,
        _ => return Err("The given button identifier could not be interpreted".to_string())
    } = action.clone();
    state.log_success(format!("Successfully configured <acc {}> click to trigger <acc {}>", button_str, action));

    Ok(())
}

pub(crate) const CLICK_MODE: Command = Command {
    execute: &execute_click_mode,
    name: "click_mode",
    accepted_arg_count: &[0, 2],
    detailed_desc: Some(concat!(
        "<green Usage: <command [no args]>>\n",
        "Show available click actions.\n",
        "<green Usage: <command [button] [action]>>\n",
        "Configure the specified button to trigger the given action. ",
        "Configurable buttons: <acc left>, <acc middle>, and <acc right>.",
    )),
    basic_desc: "Change the action triggered by mouse clicks.",
};
