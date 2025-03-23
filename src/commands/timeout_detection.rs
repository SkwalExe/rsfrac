use super::Command;
use crate::{helpers::flip_bool, AppState};

pub(crate) fn execute_timeout_detection(
    state: &mut AppState,
    _args: Vec<&str>,
) -> Result<(), String> {
    flip_bool(&mut state.render_settings.wgpu_state.disable_timeout_detection);
    state.log_info(
        if state.render_settings.wgpu_state.disable_timeout_detection {
            "GPU timeout detection disabled (not recommended)."
        } else {
            "GPU timeout detection enabled."
        },
    );
    Ok(())
}
pub(crate) const TIMEOUT_DETECTION: Command = Command {
    execute: &execute_timeout_detection,
    name: "timeout_detection",
    aliases: &["td"],
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Toggle GPU timeout detection on or off, you should leave this on unless you have a specific reason to disable it.",
};
