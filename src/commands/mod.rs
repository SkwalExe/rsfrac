use std::collections::HashMap;

use crate::app::App;
pub mod clear;
pub mod help;
pub mod pos;
pub mod quit;

pub struct Command {
    pub execute: &'static dyn Fn(&mut App, Vec<&str>),
    pub name: &'static str,
    pub basic_desc: &'static str,
    pub detailed_desc: Option<&'static str>,
    pub accepted_arg_count: &'static [usize],
}

pub fn create_commands() -> HashMap<&'static str, Command> {
    HashMap::from([
        ("clear", clear::CLEAR),
        ("quit", quit::QUIT),
        ("pos", pos::POS),
        ("help", help::HELP),
    ])
}
