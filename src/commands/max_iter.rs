use super::{command_increment::command_increment, Command};
use crate::AppState;

pub(crate) const MIN_MAX_ITER: i32 = 8;
pub(crate) const MAX_MAX_ITER: i32 = 100000;

pub(crate) fn execute_max_iter(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    let val = command_increment(
        state,
        state.render_settings.max_iter,
        args,
        MIN_MAX_ITER,
        MAX_MAX_ITER,
    )?;
    state.render_settings.max_iter = val;
    state.request_redraw();

    Ok(())
}
pub(crate) const MAX_ITER: Command = Command {
    execute: &execute_max_iter,
    name: "max_iter",
    aliases: &["mi"],
    accepted_arg_count: &[0, 1, 2],

    detailed_desc: Some(concat!(
        "<green Usage: <command +/- [value]>>\n",
        "<green Usage: <command [max_iter]>>\n",
        "<green Usage: <command [without args]>>\n",
        "- If no arguments are given, display the current iteration limit.\n",
        "- If a value is specified directly, set the iteration limit to the given value.\n",
        "- If a value is specified alongside an operator, ",
        "increase of decrease the iteration limit by the given value.\n",
        "<acc [max_iter]> must be a valid integer.",
    )),
    basic_desc:
        "Change the iteration limit used to determine if a point is converging or diverging.",
};

#[cfg(test)]
mod tests {
    use crate::{commands::max_iter::execute_max_iter, AppState};

    #[test]
    fn test_max_iter_command() {
        let mut state = AppState::default();

        // `mi -1` should return Err
        assert!(execute_max_iter(&mut state, vec!["-1"]).is_err());

        // `mi 20` should return Ok and set the max iter to 20
        execute_max_iter(&mut state, vec!["20"]).unwrap();
        assert_eq!(state.render_settings.max_iter, 20);

        // `mi + 10` should return Ok and set the max iter to 30
        execute_max_iter(&mut state, vec!["+", "10"]).unwrap();
        assert_eq!(state.render_settings.max_iter, 30);

        // `mi - 10` should return Ok and set the max iter to 25
        execute_max_iter(&mut state, vec!["-", "5"]).unwrap();
        assert_eq!(state.render_settings.max_iter, 25);

        // `mi - 100` should return Err
        assert!(execute_max_iter(&mut state, vec!["-", "100"]).is_err());

        // `mi blahblah 100` should return Err
        assert!(execute_max_iter(&mut state, vec!["blahblah", "100"]).is_err());

        // max_iter should have remained to 25
        assert_eq!(state.render_settings.max_iter, 25);
    }
}
