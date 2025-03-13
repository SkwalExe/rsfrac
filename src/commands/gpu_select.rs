use std::sync::{LazyLock, Mutex};

use wgpu::{Adapter, Backends};

use super::Command;
use crate::{helpers::markup::esc, AppState};

fn get_adapter_description(adapter: &Adapter) -> String {
    let info = adapter.get_info();

    format!(
        "{}, driver:{}, backend:{}",
        info.name, info.driver, info.backend
    )
}

pub(crate) fn execute_gpu_select(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if !state.render_settings.use_gpu {
        return Err("GPU mode must be enabled first.".to_string());
    }

    static DETECTED_ADAPTERS: LazyLock<Mutex<Vec<Adapter>>> =
        LazyLock::new(|| Mutex::new(Vec::new()));

    let mut locked = DETECTED_ADAPTERS.lock().unwrap();

    if args.is_empty() {
        *locked = state
            .render_settings
            .wgpu_state
            .instance
            .as_ref()
            .unwrap()
            .enumerate_adapters(Backends::all());

        state.log_info_title(
            "Detected adapters",
            locked
                .iter()
                .enumerate()
                .map(|(i, adapter)| format!("<acc {i}>: {}", esc(get_adapter_description(adapter))))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        return Ok(());
    }

    // check if the provided arg is parsable as an int
    let index = args[0]
        .parse::<usize>()
        .map_err(|_| format!("The provided argment could not be parsed as an integer."))?;

    if index >= locked.len() {
        return Err(
            "Provided index is not associated with any adapter in the adapter list.".to_string(),
        );
    }

    let adapter = locked.remove(index);

    state
        .render_settings
        .select_adapter_sync(adapter, None)
        .map_err(|e| {
            state.render_settings.use_gpu = false;
            e
        })?;

    state.log_success("Successfully selected adapter.".to_string());
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
