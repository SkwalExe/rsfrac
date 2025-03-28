use futures::executor::block_on;
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
    // If no arguments are provided, show detect and display the visible adapters.
    if args.is_empty() {
        let detected_adapters_string = state
            .render_settings
            .wgpu_state
            .detected_adapters()
            .iter()
            .enumerate()
            .map(|(i, adapter)| format!("<acc {i}>: {}", esc(get_adapter_description(adapter))))
            .collect::<Vec<_>>()
            .join("\n");
        state.log_info_title("Detected adapters", detected_adapters_string);
        return Ok(());
    }

    // Check if at least one adapter has been detected.
    if state
        .render_settings
        .wgpu_state
        .detected_adapters()
        .is_empty()
    {
        return Err("No adapter has been detected. Cannot proceed.".to_string());
    }

    // check if the provided arg is parsable as an int
    let index = args[0]
        .parse::<usize>()
        .map_err(|_| "The provided argument could not be parsed as an integer.".to_string())?;

    // Check if an adapter is associated with the provided index.
    if index >= state.render_settings.wgpu_state.detected_adapters().len() {
        return Err(
            "Provided index is not associated with any adapter in the adapter list.".to_string(),
        );
    }

    // Effectively select the adapter.
    block_on(state.render_settings.wgpu_state.set_preferred_adapter(index, None)).inspect_err(|e| {
        state.log_error(
            format!(
                "GPU mode has been disabled because the adapter selected failed due to the following error: {}", 
                esc(e)
            )
        );
        state.render_settings.wgpu_state.use_gpu = false;
    })?;

    // get the adapter info before selecting it because we would lose ownership
    let selected_gpu_info = esc(get_adapter_description(
        state.render_settings.wgpu_state.get_adapter()?,
    ));

    // If cpu mode was disabled, enable it
    state.render_settings.wgpu_state.use_gpu = true;

    state.log_success(format!(
        "Successfully enabled GPU mode with adapter: {selected_gpu_info}."
    ));

    state.request_redraw();
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
        "Enable GPU mode and select the processor with the specified index.",
    )),
    basic_desc:
        "Allows you to select which GPU will be used for rendering the canvas and screenshots.",
};
