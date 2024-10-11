pub fn execute_clear(app: &mut crate::app::App, _args: Vec<&str>) {
    app.log_messages.clear();
}
