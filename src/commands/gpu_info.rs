use super::Command;
use crate::{helpers::markup::esc, AppState};

pub(crate) fn execute_gpu_info(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    if !state.render_settings.use_gpu {
        state.log_info(
            "GPU mode is disabled. You can try to enable it with the <command gpu> command.",
        );
        return Ok(());
    }
    let info = state
        .render_settings
        .wgpu_state
        .adapter
        .as_ref()
        .unwrap()
        .get_info();
    state.log_info_title(
        "GPU INFO", 
        format!(
            "<blue Name>: {}\n<blue VendorID>: {}\n<blue DeviceID>: {}\n<blue DeviceType>: {}\n<blue Driver>: {}\n<blue DriverInfo>: {}\n<blue Backend>: {}", 
            esc(info.name), 
            esc(info.vendor), 
            esc(info.device), 
            esc(format!("{:?}", info.device_type)), 
            esc(info.driver), 
            esc(info.driver_info), 
            esc(info.backend)
    ));
    Ok(())
}

pub(crate) const GPU_INFO: Command = Command {
    execute: &execute_gpu_info,
    name: "gpu_info",
    aliases: &["gpui", "gi"],
    accepted_arg_count: &[0],
    basic_desc: "Provides details about the GPU in use.",
    detailed_desc: None,
};
