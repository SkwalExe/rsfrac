pub fn execute_quit(app: &mut crate::app::App, _args: Vec<&str>) {
    app.quit = true;
}
