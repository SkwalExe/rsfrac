use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app_state::ClickMode,
    helpers::{Focus, ZoomDirection},
    App,
};

use super::Canvas;
impl Canvas<'_> {
    pub(crate) fn handle_mouse_event(app: &mut App, event: MouseEvent) {
        app.app_state.focused = Focus::Canvas;

        // first, convert the key press position to canvas coordinates

        let canvas_pos = app
            .app_state
            .render_settings
            .ratatui_to_canvas_coords(event.column, event.row);

        let action = match event.kind {
            MouseEventKind::Down(MouseButton::Left) => &app.app_state.click_config.left,
            MouseEventKind::Down(MouseButton::Right) => &app.app_state.click_config.right,
            MouseEventKind::Down(MouseButton::Middle) => &app.app_state.click_config.right,
            _ => return,
        };
        match action {
            ClickMode::Move => {
                app.app_state.render_settings.pos =
                    app.app_state.render_settings.coord_to_c(canvas_pos);
                app.app_state.request_redraw();
            }
            ClickMode::ZoomOut => {
                app.app_state.zoom_at(canvas_pos, ZoomDirection::Out);
                app.app_state.request_redraw();
            }
            ClickMode::ZoomIn => {
                app.app_state.zoom_at(canvas_pos, ZoomDirection::In);
                app.app_state.request_redraw();
            }
            ClickMode::JuliaConstant => {
                app.app_state.render_settings.julia_constant =
                    app.app_state.render_settings.coord_to_c(canvas_pos);

                app.app_state.request_redraw();
            }
            ClickMode::MandelConstant => {
                app.app_state.render_settings.mandel_constant =
                    app.app_state.render_settings.coord_to_c(canvas_pos);
                app.app_state.request_redraw();
            }
            ClickMode::BailOut => {
                app.app_state.render_settings.bailout = app
                    .app_state
                    .render_settings
                    .coord_to_c(canvas_pos)
                    .abs()
                    .real()
                    .to_f32();
                app.app_state.request_redraw();
            }
            ClickMode::Info => {
                let point = app.app_state.render_settings.coord_to_c(canvas_pos.clone());

                // prevent subtracting with overflow
                if event.column == 0 || event.row == 0 {
                    return;
                }

                // Try to access the diverg value at this point or return prematurely.
                let Some(Some(diverg)) = app
                    .diverg_matrix
                    .get(event.row as usize * 2 - 2)
                    .map(|l| l.get(event.column as usize - 1))
                else {
                    return;
                };

                app.app_state.log_info_title(
                    "Click Info",
                    format!(
                        "Real: <acc {}>\nImag: <acc {}>\nDiverg: <acc {}>",
                        point.real().to_f32(),
                        point.imag().to_f32(),
                        diverg,
                    ),
                )
            }
        }
    }
}
