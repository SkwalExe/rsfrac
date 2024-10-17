use std::collections::HashMap;

use crate::app::App;
pub mod clear;
pub mod command_increment;
pub mod help;
pub mod max_iter;
pub mod move_dist;
pub mod pos;
pub mod prec;
pub mod quit;
pub mod frac;
pub mod zoom_factor;
pub mod color;
pub mod version;

pub struct Command {
    pub execute: &'static dyn Fn(&mut App, Vec<&str>),
    pub name: &'static str,
    pub basic_desc: &'static str,
    pub detailed_desc: Option<&'static str>,
    pub accepted_arg_count: &'static [usize],
}

pub fn create_commands() -> HashMap<&'static str, &'static Command> {
    HashMap::from([
        (clear::CLEAR.name, &clear::CLEAR),
        (quit::QUIT.name, &quit::QUIT),
        (pos::POS.name, &pos::POS),
        (help::HELP.name, &help::HELP),
        (zoom_factor::ZOOM_FACTOR.name, &zoom_factor::ZOOM_FACTOR),
        (prec::PREC.name, &prec::PREC),
        (max_iter::MAX_ITER.name, &max_iter::MAX_ITER),
        (move_dist::MOVE_DIST.name, &move_dist::MOVE_DIST),
        (version::VERSION_COMMAND.name, &version::VERSION_COMMAND),
        (color::COLOR.name, &color::COLOR),
        (frac::FRAC.name, &frac::FRAC),
    ])
}
