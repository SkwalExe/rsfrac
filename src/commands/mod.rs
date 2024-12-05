//! Contains the command system logic, as well as the code for each available command.

use crate::AppState;
pub(crate) mod capture;
pub(crate) mod capture_fit;
pub(crate) mod capture_format;
pub(crate) mod capture_hq;
pub(crate) mod clear;
pub(crate) mod click_mode;
pub(crate) mod color;
pub(crate) mod command_increment;
pub(crate) mod frac;
pub(crate) mod gpu;
pub(crate) mod help;
pub(crate) mod load;
pub(crate) mod max_iter;
pub(crate) mod move_dist;
pub(crate) mod pos;
pub(crate) mod prec;
pub(crate) mod quit;
pub(crate) mod save;
pub(crate) mod smoothness;
pub(crate) mod version;
pub(crate) mod zoom_factor;

type CommandClos = &'static dyn Fn(&mut AppState, Vec<&str>) -> Result<(), String>;

/// Represents a rsfrac command.
pub(crate) struct Command {
    /// Closure to call to execute the command.
    pub(crate) execute: CommandClos,
    /// The name of the command.
    pub(crate) name: &'static str,
    pub(crate) aliases: &'static [&'static str],
    /// A basic description of the command.
    pub(crate) basic_desc: &'static str,
    /// An optional detailed description of the command.
    pub(crate) detailed_desc: Option<&'static str>,
    /// The list of accepted argument count.
    pub(crate) accepted_arg_count: &'static [usize],
}

pub(crate) fn get_commands_list() -> [&'static Command; 20] {
    [
        &help::HELP,
        &quit::QUIT,
        &clear::CLEAR,
        &version::VERSION_COMMAND,
        &save::SAVE,
        &load::LOAD,
        &capture::CAPTURE,
        &capture_fit::CAPTURE_FIT,
        &capture_hq::CAPTURE_HQ,
        &capture_format::CAPTURE_FORMAT,
        &gpu::GPU,
        &pos::POS,
        &prec::PREC,
        &max_iter::MAX_ITER,
        &color::COLOR,
        &smoothness::SMOOTHNESS,
        &frac::FRAC,
        &zoom_factor::ZOOM_FACTOR,
        &move_dist::MOVE_DIST,
        &click_mode::CLICK_MODE,
    ]
}

pub(crate) fn get_command_by_name(name: &str) -> Option<&'static Command> {
    let name = name.to_lowercase();
    get_commands_list()
        .iter()
        .find(|c| c.name == name || c.aliases.contains(&name.as_str()))
        .map(|c| &**c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_by_name() {
        assert_eq!(get_command_by_name("help").unwrap().name, "help");
        assert_eq!(get_command_by_name("hElP").unwrap().name, "help");
        assert_eq!(get_command_by_name("h").unwrap().name, "help");
        assert_eq!(get_command_by_name("Q").unwrap().name, "quit");
        assert_eq!(get_command_by_name("mi").unwrap().name, "max_iter");
        assert_eq!(get_command_by_name("cP").unwrap().name, "capture");
        assert_eq!(get_command_by_name("c").unwrap().name, "clear");
        assert_eq!(get_command_by_name("cpf").unwrap().name, "capture_fit");

        assert!(get_command_by_name("blabla").is_none());
    }
}
