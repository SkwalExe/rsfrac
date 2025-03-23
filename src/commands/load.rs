use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};

use super::Command;
use crate::{
    commands::save::SAVE_EXTENSION,
    helpers::{markup::esc, SavedState},
    AppState,
};
const MAX_SEARCH_DEPTH: i32 = 10;

/// Returns an error message when the max depth has just been exceeded.
fn find_state_files(path: PathBuf, depth: i32) -> Result<(Vec<PathBuf>, i32), String> {
    // eprintln!("Searching in directory: {}", path.to_string_lossy());
    let mut results: Vec<PathBuf> = vec![];
    let mut searched_directories = 0;

    fs::read_dir(&path)
        .map_err(|err| {
            format!(
                "Cannot list directory: {}: {err}",
                path.to_owned().to_string_lossy()
            )
        })?
        // Only keep sane entries
        .filter_map(|entry| entry.ok())
        .for_each(|entry| {
            // Ignore entries with an invalid file type.
            let Ok(filetype) = entry.file_type() else {
                return;
            };

            if filetype.is_file()
                && entry
                    .file_name()
                    .to_string_lossy()
                    .ends_with(SAVE_EXTENSION)
            {
                results.push(entry.path());
                // eprintln!("Detected state file: {}", entry.path().to_string_lossy())
            } else if filetype.is_dir() && depth < MAX_SEARCH_DEPTH {
                if let Ok((mut files, searched_dirs)) = find_state_files(entry.path(), depth + 1) {
                    searched_directories += searched_dirs + 1;
                    results.append(&mut files);
                }
            }
        });

    Ok((results, searched_directories))
}

pub(crate) fn execute_load(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    static DETECTED_FILES: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
    static CURRENT_STATE_FILE_INDEX: Mutex<usize> = Mutex::new(0);
    let mut detected_files_locked = DETECTED_FILES
        .lock()
        .map_err(|e| format!("ERROR: Could not access detect detected files list: {e}"))?;
    let mut current_state_file_index_locked = CURRENT_STATE_FILE_INDEX
        .lock()
        .map_err(|e| format!("ERROR: Could not access current state file index: {e}"))?;

    // If no arguments are passed, detect state files.
    if args.is_empty() {
        *current_state_file_index_locked = 0;
        let searched_dirs;
        (*detected_files_locked, searched_dirs) = find_state_files(PathBuf::from("."), 0)?;
        detected_files_locked.sort_unstable();

        state.log_info(if detected_files_locked.is_empty() {
            format!("Looked into <acc {searched_dirs}> directories, but no state file was found.")
        } else {
            format!(
                "Looked into <acc {searched_dirs}> directories, the following state files have been detected:\n{}",
                {
                    let mut res = String::new();
                    for (i, filename) in detected_files_locked.iter().enumerate() {
                        res += &format!("<acc {i}>: {}\n", esc(filename.to_string_lossy()));
                    }
                    res.trim().to_string()
                }
            )
        });
        return Ok(());
    }

    // From here, at least one argument has been passed.

    let mut filename = args[0].to_string();

    if filename == "cycle" {
        if detected_files_locked.is_empty() {
            return Err(concat!(
                "No state file has been found in your current working directory, ",
                "or the detection has not been performed yet. ",
                "To detect state files, run <command load> with no arugments."
            )
            .to_string());
        }

        filename = current_state_file_index_locked.to_string();
        *current_state_file_index_locked =
            (*current_state_file_index_locked + 1) % detected_files_locked.len();
    }

    // Check if the provided filename can be parsed to an integer
    if let Ok(num) = filename.parse::<usize>() {
        // Check if the detected files vector can be indexed by this integer
        if let Some(filename_) = detected_files_locked.get(num) {
            filename = filename_.to_string_lossy().to_string();
        }
    }

    let mut file = File::open(&filename).map_err(|err| {
        format!(
            "Could not open file <command {}>: {}",
            esc(&filename),
            esc(err)
        )
    })?;

    let mut str = String::new();
    file.read_to_string(&mut str)
        .map_err(|err| format!("The file cannot be read: {}", esc(err)))?;

    let saved: SavedState = SavedState::from_str(&str)?;
    state.apply(saved, &filename);
    Ok(())
}

pub(crate) const LOAD: Command = Command {
    execute: &execute_load,
    name: "load",
    aliases: &["l"],
    accepted_arg_count: &[0, 1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [no args]>>\n",
        "When no args are provided, the state files with the <command rsf> extension in the current ",
        "working directory will be detected and loaded into a list.\n",
        "<green Usage: <command [integer]>>\n",
        "Load the state file corresponding to the provided number in the detection list. ",
        "Must be ran after <command load>.\n",
        "<green Usage: <command cycle>>\n",
        "Cycle through the state files detected with <command load>. ",
        "After running the command one time, ",
        "you can rapidly cycle through all the state files using ",
        "<command Ctrl+R>, which repeats the last command. \n",
        "<green Usage: <command [file path]>>\n",
        "Load the state from the specified path.\n",
    )),
    basic_desc: concat!(
        "Restore the app state (canvas size, position...) from a ",
        "file that was previously created with the <command save> command."
    ),
};
