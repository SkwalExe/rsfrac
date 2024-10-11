use clear::execute_clear;
use pos::execute_pos;
use quit::execute_quit;

use crate::app::App;
pub mod clear;
pub mod pos;
pub mod quit;

type Command = &'static dyn Fn(&mut App, Vec<&str>);
pub fn get_command(command: &str) -> Option<Command> {
    Some(match command {
        "clear" => &execute_clear,
        "quit" => &execute_quit,
        "pos" => &execute_pos,
        _ => return None,
    })
}
