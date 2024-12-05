use super::Command;
use crate::colors::{get_palette_index_by_name, COLORS};
use crate::AppState;

pub(crate) fn execute_color(state: &mut AppState, args: Vec<&str>) -> Result<(), String> {
    if args.is_empty() {
        state.log_raw(format!(
            "Current colors: <acc {}>\nAvailable colors: {}",
            state.render_settings.get_palette().name,
            COLORS
                .iter()
                .map(|col| format!("<acc {}>", col.name))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        return Ok(());
    }

    let pal = get_palette_index_by_name(args[0])
        .ok_or(format!("Could not find palette: <red {}>", args[0]))?;

    state.render_settings.palette_index = pal;
    state.log_success(format!("Selected color scheme: <acc {}>", COLORS[pal].name,));
    state.request_repaint();

    Ok(())
}

pub(crate) const COLOR: Command = Command {
    execute: &execute_color,
    name: "color",
    aliases: &["co"],
    accepted_arg_count: &[0, 1],
    detailed_desc: Some(concat!(
        "<green Usage: <command [color]>>\n",
        "<green Usage: <command [without args]>>\n",
        "If no argument is given, display the available color schemes. ",
        "Else, select the specified color scheme.",
    )),
    basic_desc: "List available color schemes or select the specified one.",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_color() {
        let mut state = AppState::default();

        // `color non_exist` should fail.
        assert!(execute_color(&mut state, vec!["non_exist"]).is_err());

        // `color iceberg` should not fail and the iceberg palette should be selected.
        execute_color(&mut state, vec!["iceberg"]).unwrap();
        assert_eq!(
            state.render_settings.palette_index,
            get_palette_index_by_name("iceberg").unwrap()
        );

        // `color for` should not fail and the forest palette should be selected.
        execute_color(&mut state, vec!["for"]).unwrap();
        assert_eq!(
            state.render_settings.palette_index,
            get_palette_index_by_name("forest").unwrap()
        );
    }
}
