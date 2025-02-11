use std::{
    fs::{self, File},
    io::Read,
    str::FromStr,
    sync::Mutex,
};

use super::Command;
use crate::{commands::save::SAVE_EXTENSION, helpers::SavedState, AppState};
const MAX_SEARCH_DEPTH: i32 = 10;

fn find_state_files(path: &str, depth: i32) -> Result<Vec<String>, String> {
    // eprintln!("Depth: {depth} -> Looking for state files in {path}");
    if depth > MAX_SEARCH_DEPTH {
        return Err(String::from("Max depth exceeded."));
    }

    Ok(fs::read_dir(&path)
        .map_err(|err| format!("Cannot list directory: {}: {err}", path))?
        // Only keep sane entries
        .filter_map(|entry| entry.ok())
        // Only keep files and directories
        .filter(|entry| {
            entry.file_type().is_ok()
            // UNWRAP: is_ok() checked above
                && (entry.file_type().unwrap().is_file() || entry.file_type().unwrap().is_dir())
        })
        // Look into subdirectories
        .flat_map(|entry| {
            let entry_path = entry.path().to_string_lossy().into_owned();
            // UNWRAP: is_ok() checked above
            if entry.file_type().unwrap().is_file() {
                Vec::from([entry_path])
            } else {
                find_state_files(&entry_path, depth + 1).unwrap_or(Vec::new())
            }
        })
        // Only keep files ending in .rsf
        .filter(|filename| filename.ends_with(SAVE_EXTENSION))
        .collect::<Vec<_>>())
}

pub(crate) fn execute_load(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    static DETECTED_FILES: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static CURRENT_STATE_FILE_INDEX: Mutex<usize> = Mutex::new(0);
    if args.is_empty() {
        *CURRENT_STATE_FILE_INDEX.lock().unwrap() = 0;
        let mut locked = DETECTED_FILES.lock().unwrap();
        *locked = find_state_files(".", 0)?;
        locked.sort_unstable();

        state.log_info(if locked.is_empty() {
            "No state file detected in your current working directory.".to_string()
        } else {
            format!(
                "These state files have been detected in your current working directory:\n{}",
                {
                    let mut res = String::new();
                    for (i, filename) in locked.iter().enumerate() {
                        res += &format!("<acc {i}>: {filename}\n");
                    }
                    res.trim().to_string()
                }
            )
        });
        return Ok(());
    }

    let mut filename = args[0].to_string();

    if filename == "cycle" {
        let files_ = DETECTED_FILES.lock().unwrap();
        if files_.is_empty() {
            return Err(concat!(
                "No state file has been found in your current working directory, ",
                "or the detection has not been performed yet. ",
                "To detect state files, run <command load> with no arugments."
            )
            .to_string());
        }

        let mut index = CURRENT_STATE_FILE_INDEX.lock().unwrap();

        filename = format!("{index}");

        *index = (*index + 1) % files_.len();
    }

    // Check if the provided filename can be parsed to an integer
    if let Ok(num) = filename.parse::<usize>() {
        let locked = DETECTED_FILES.lock().unwrap();
        // Check if the detected files vector can be indexed by this integer
        if let Some(filename_) = locked.get(num) {
            filename = filename_.to_string();
        }
    }

    let mut file = File::open(&filename)
        .map_err(|err| format!("Could not open file <command {filename}>: {err}"))?;

    let mut str = String::new();
    file.read_to_string(&mut str)
        .map_err(|err| format!("The file cannot be read: {err}"))?;

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
