use std::{
    fs::{self, File},
    io::Read,
    sync::Mutex,
};

use super::Command;
use crate::{helpers::SavedState, AppState};

pub(crate) fn execute_load(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    static DETECTED_FILES: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static CURRENT_STATE_FILE_INDEX: Mutex<usize> = Mutex::new(0);
    if args.is_empty() {
        *CURRENT_STATE_FILE_INDEX.lock().unwrap() = 0;
        let mut locked = DETECTED_FILES.lock().unwrap();
        *locked = fs::read_dir(".")
            .map_err(|err| format!("Cannot list current working directory: {err}"))?
            // Only keep sane entries
            .filter_map(|entry| entry.ok())
            // Only keep files not directories
            .filter(|entry| {
                let ft = entry.file_type();
                ft.is_ok() && ft.unwrap().is_file()
            })
            // Only keep files with valid unicode names
            .filter_map(|entry| entry.file_name().into_string().ok())
            // Only keep files ending in .rsf
            .filter(|filename| filename.ends_with(".rsf"))
            .collect::<Vec<_>>();

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
    let saved: SavedState =
        toml::from_str(&str).map_err(|err| format!("Could not parse the provided file: {err}"))?;
    state.apply(saved, &filename);
    Ok(())
}

pub(crate) const LOAD: Command = Command {
    execute: &execute_load,
    name: "load",
    accepted_arg_count: &[0, 1],
    detailed_desc: None,
    basic_desc: concat!(
        "Restore the app state (canvas size, position...) from a ",
        "file that was previously created with the <command save> command."
    ),
};
