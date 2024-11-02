use std::collections::HashMap;

use crate::AppState;
pub(crate) mod capture;
pub(crate) mod clear;
pub(crate) mod color;
pub(crate) mod command_increment;
pub(crate) mod frac;
pub(crate) mod help;
pub(crate) mod max_iter;
pub(crate) mod move_dist;
pub(crate) mod pos;
pub(crate) mod prec;
pub(crate) mod quit;
pub(crate) mod version;
pub(crate) mod zoom_factor;

/// Represents a rsfrac command.
pub(crate) struct Command {
    /// Closure to call to execute the command.
    pub(crate) execute: &'static dyn Fn(&mut AppState, Vec<&str>),
    /// The name of the command.
    pub(crate) name: &'static str,
    /// A basic description of the command.
    pub(crate) basic_desc: &'static str,
    /// An optional detailed description of the command.
    pub(crate) detailed_desc: Option<&'static str>,
    /// The list of accepted argument count.
    pub(crate) accepted_arg_count: &'static [usize],
}

/// Returns a `HashMap` associating each command's name to itself.
pub(crate) fn get_commands() -> HashMap<&'static str, &'static Command> {
    HashMap::from([
        (help::HELP.name, &help::HELP),
        (quit::QUIT.name, &quit::QUIT),
        (clear::CLEAR.name, &clear::CLEAR),
        (version::VERSION_COMMAND.name, &version::VERSION_COMMAND),
        (capture::CAPTURE.name, &capture::CAPTURE),
        (pos::POS.name, &pos::POS),
        (prec::PREC.name, &prec::PREC),
        (max_iter::MAX_ITER.name, &max_iter::MAX_ITER),
        (color::COLOR.name, &color::COLOR),
        (frac::FRAC.name, &frac::FRAC),
        (zoom_factor::ZOOM_FACTOR.name, &zoom_factor::ZOOM_FACTOR),
        (move_dist::MOVE_DIST.name, &move_dist::MOVE_DIST),
    ])
}
