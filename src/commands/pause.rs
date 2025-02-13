use super::Command;
use crate::AppState;

pub(crate) fn execute_pause(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    state.pause_jobs = !state.pause_jobs;
    state.log_info(if state.pause_jobs {
        "Job execution paused."
    } else {
        "Job execution resumed."
    });
    Ok(())
}
pub(crate) const PAUSE: Command = Command {
    execute: &execute_pause,
    name: "pause",
    aliases: &["p"],
    accepted_arg_count: &[0],
    detailed_desc: Some(concat!(
        "Can be useful if you want to initialize multiple screenshots without ",
        "being disturbed when the application freezes due to writing the result to disk."
    )),
    basic_desc: "Pause or resume parallel job execution, such as screenshots.",
};
