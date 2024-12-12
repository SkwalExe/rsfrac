use super::Command;
use crate::AppState;

const MIN_LIMIT: i32 = 1;
const MAX_LIMIT: i32 = 10000;

pub(crate) fn execute_chunk_size(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        state.log_info(format!(
            "Current render chunk size limit (0 = No limit): <acc {}>",
            state.render_settings.chunk_size_limit.unwrap_or(0)
        ));
        return Ok(());
    }

    if args[0].to_lowercase() == "reset" {
        state.render_settings.chunk_size_limit = None;
        return Ok(());
    }

    if let Ok(new_val) = args[0].parse::<i32>() {
        if (MIN_LIMIT..MAX_LIMIT).contains(&new_val) {
            state.render_settings.chunk_size_limit = Some(new_val);
            state.log_success(format!(
                "Successfully set chunk size limit to <acc {}>.",
                new_val
            ));
            return Ok(());
        }
    }
    Err(format!(
        "Invalid limit provided. Make sure to enter a valid integer between {} and {}.",
        MIN_LIMIT, MAX_LIMIT
    ))
}
pub(crate) const CHUNK_SIZE: Command = Command {
    execute: &execute_chunk_size,
    name: "chunk_size",
    aliases: &["cs"],
    accepted_arg_count: &[0, 1],

    detailed_desc: Some(concat!(
        "This is sometimes needed because the GPU can time out under a huge computational, ",
        "this returning incorrect results. Changing the maximum chunk size can help reduce ",
        "render passes duration, therefore preventing timeouts and inconsistent results. ",
        "There are as of today no WGPU apis allowing to know if the GPU job finished or timed out.\n",
        "<green Usage: <command [no args]>>\n",
        "- Displays the current value\n",
        "<green Usage: <command reset>>\n",
        "- Reverts the limit to its default value, which is none (no limit).\n",
        "<green Usage: <command [value]>>\n",
        "- Sets the limit to <acc [value]>. ",
        "<acc [value]> must be a valid integer between <acc 1> and <acc 10000>.",
    )),
    basic_desc:
        "Changes the maximum number of lines to render per chunk (use <command help cs> for more info). "
};
