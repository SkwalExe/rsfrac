pub fn execute_pos(app: &mut crate::app::App, args: Vec<&str>) {
    if args.is_empty() {
        app.log_info_title(
            "Current Position",
            format!(
                "Real: {}\nImag: {}",
                app.render_settings.pos.real(),
                app.render_settings.pos.imag()
            ),
        );
    } else if args.len() == 2 {
    }
}
