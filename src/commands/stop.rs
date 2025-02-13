use super::Command;
use crate::AppState;

pub(crate) fn execute_stop(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    state.remove_jobs = true;
    state.log_info("All jobs have been removed.");
    Ok(())
}
pub(crate) const STOP: Command = Command {
    execute: &execute_stop,
    name: "stop",
    aliases: &["s"],
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Removes all screenshots from the job queue.",
};
