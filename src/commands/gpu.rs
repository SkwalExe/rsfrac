use futures::executor::block_on;

use super::Command;
use crate::AppState;

pub(crate) fn execute_gpu(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    if state.render_settings.wgpu_state.use_gpu {
        state.render_settings.wgpu_state.use_gpu = false;
        state.log_info("GPU mode disabled.");
    } else {
        block_on(state.render_settings.initialize_gpu(None))
            .map_err(|err| format!("GPU mode could not be enabled: {err}"))?;
        state.log_success("GPU mode initialized successfully! To benefit from high precision arithmetic you will have to disable it with the <command gpu> command.");
        state.render_settings.wgpu_state.use_gpu = true;
    }
    state.request_redraw();
    Ok(())
}
pub(crate) const GPU: Command = Command {
    execute: &execute_gpu,
    name: "gpu",
    aliases: &[],
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: concat!(
        "Switch GPU mode on or off. In GPU mode, renders are done using the parallel ",
        "computing capabilities of your hardware, which greatly improves rendering speeds. ",
        "Please note that floating point precision is limited to 32 bits for now, but arbitrary ",
        "precision will be implemementd soon."
    ),
};
