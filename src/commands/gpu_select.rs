use wgpu::Adapter;

use super::Command;
use crate::{helpers::markup::esc, AppState};

/// Returns a string with basic info about an adapter.
fn get_adapter_description(adapter: &Adapter) -> String {
    let info = adapter.get_info();

    format!(
        "{}, driver:{}, backend:{}",
        info.name, info.driver, info.backend
    )
}

pub(crate) fn execute_gpu_select(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    // Require GPU mode to be enabled first.
    if !state.render_settings.wgpu_state.use_gpu {
        return Err("GPU mode must be enabled first.".to_string());
    }

    // If no arguments are provided, show detect and display the visible adapters.
    if args.is_empty() {
        state.detect_adapters();

        state.log_info_title(
            "Detected adapters",
            state
                .detected_adapters
                .iter()
                .enumerate()
                .map(|(i, adapter)| format!("<acc {i}>: {}", esc(get_adapter_description(adapter))))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        return Ok(());
    }

    // Check if at least one adapter has been detected.
    if state.detected_adapters.is_empty() {
        return Err(
            "No adapter has been detected yet. You must run <command gpu-select> at least once."
                .to_string(),
        );
    }

    // check if the provided arg is parsable as an int
    let index = args[0]
        .parse::<usize>()
        .map_err(|_| "The provided argument could not be parsed as an integer.".to_string())?;

    // Check if an adapter is associated with the provided index.
    if index >= state.detected_adapters.len() {
        return Err(
            "Provided index is not associated with any adapter in the adapter list.".to_string(),
        );
    }

    let adapter = state.detected_adapters.remove(index);
    // Refresh the adapter list since we just removed one.
    state.detect_adapters();

    // get the adapter info before selecting it because we would lose ownership
    let selected_gpu_info = esc(get_adapter_description(&adapter));

    // Effectively select the adapter.
    state
        .render_settings
        .select_adapter_sync(adapter, None)
        // If an error was encountered, disable GPU mode and return the error.
        .inspect_err(|_| state.render_settings.wgpu_state.use_gpu = false)?;

    state.log_success(format!(
        "Successfully selected adapter: {selected_gpu_info}."
    ));
    Ok(())
}

pub(crate) const GPU_SELECT: Command = Command {
    execute: &execute_gpu_select,
    name: "gpu_select",
    aliases: &["gpus", "gs"],
    accepted_arg_count: &[0, 1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [no args]>>\n",
        "When no args are provided, all the available graphical processors ",
        "will be listed and associated with a unique index number.\n",
        "<green Usage: <command [integer]>>\n",
        "Select the GPU with the specified index. ",
        "Must be ran after <command gpu_select> alone.\n",
    )),
    basic_desc:
        "Allows you to select which GPU will be used for rendering the canvas and screenshots.",
};
