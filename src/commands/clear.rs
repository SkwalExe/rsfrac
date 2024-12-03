use super::Command;
use crate::AppState;

pub(crate) fn execute_clear(state: &mut AppState, _args: Vec<&str>) -> Result<(), String> {
    state.log_messages.clear();
    Ok(())
}

pub(crate) const CLEAR: Command = Command {
    execute: &execute_clear,
    name: "clear",
    aliases: &["c"],
    accepted_arg_count: &[0],
    detailed_desc: None,
    basic_desc: "Clear all messages from the log panel.",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_command() {
        let mut state = AppState::default();
        state.log_info("This is an info message");
        execute_clear(&mut state, vec![]).unwrap();
        assert_eq!(state.log_messages.len(), 0);
    }
}
