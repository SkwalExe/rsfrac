use super::Command;
use crate::AppState;

pub(crate) fn execute_gpu(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    if state.render_settings.use_gpu {
        state.render_settings.use_gpu = false;
        state.log_info("GPU mode disabled.");
    } else {
        state
            .render_settings
            .initialize_gpu_sync(None)
            .map_err(|err| format!("GPU mode could not be enabled: {err}"))?;
        state.log_success("GPU mode initialized successfully!");
        state.render_settings.use_gpu = true;
    }
    state.request_redraw();
    Ok(())
}
pub(crate) const GPU: Command = Command {
    execute: &execute_gpu,
    name: "gpu",
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: concat!(
        "Switch GPU mode on or off. In GPU mode, renders are done using the parallel ",
        "computing capabilities of your hardware, which greatly improves rendering speeds. ",
        "Please note that floating point precision is limited to 32 bits for now, but arbitrary ",
        "precision will be implemementd soon."
    ),
};