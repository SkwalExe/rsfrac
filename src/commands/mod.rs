use std::collections::HashMap;

use crate::AppState;
pub(crate) mod capture;
pub(crate) mod capture_format;
pub(crate) mod clear;
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
pub(crate) mod version;
pub(crate) mod zoom_factor;

type CommandClos = &'static dyn Fn(&mut AppState, Vec<&str>) -> Result<(), String>;

/// Represents a rsfrac command.
pub(crate) struct Command {
    /// Closure to call to execute the command.
    pub(crate) execute: CommandClos,
    /// The name of the command.
    pub(crate) name: &'static str,
    /// A basic description of the command.
    pub(crate) basic_desc: &'static str,
    /// An optional detailed description of the command.
    pub(crate) detailed_desc: Option<&'static str>,
    /// The list of accepted argument count.
    pub(crate) accepted_arg_count: &'static [usize],
}

pub(crate) fn get_commands_list() -> [&'static Command; 16] {
    [
        &help::HELP,
        &quit::QUIT,
        &clear::CLEAR,
        &version::VERSION_COMMAND,
        &save::SAVE,
        &load::LOAD,
        &capture::CAPTURE,
        &capture_format::CAPTURE_FORMAT,
        &gpu::GPU,
        &pos::POS,
        &prec::PREC,
        &max_iter::MAX_ITER,
        &color::COLOR,
        &frac::FRAC,
        &zoom_factor::ZOOM_FACTOR,
        &move_dist::MOVE_DIST,
    ]
}

/// Returns a `HashMap` associating each command's name to itself.
pub(crate) fn get_commands_map() -> HashMap<&'static str, &'static Command> {
    HashMap::from(get_commands_list().map(|command| (command.name, command)))
}
