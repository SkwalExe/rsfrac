use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app_state::ClickMode,
    helpers::{Focus, ZoomDirection},
    AppState,
};

use super::Canvas;
impl Canvas<'_> {
    pub(crate) fn handle_mouse_event(state: &mut AppState, event: MouseEvent) {
        state.focused = Focus::Canvas;

        // first, convert the key press position to canvas coordinates

        let canvas_pos = state
            .render_settings
            .ratatui_to_canvas_coords(event.column, event.row);

        let action = match event.kind {
            MouseEventKind::Down(MouseButton::Left) => &state.click_config.left,
            MouseEventKind::Down(MouseButton::Right) => &state.click_config.right,
            MouseEventKind::Down(MouseButton::Middle) => &state.click_config.right,
            _ => return,
        };
        match action {
            ClickMode::Move => {
                state.render_settings.pos = state.render_settings.coord_to_c(canvas_pos)
            }
            ClickMode::ZoomOut => {
                state.zoom_at(canvas_pos, ZoomDirection::Out);
            }
            ClickMode::ZoomIn => {
                state.zoom_at(canvas_pos, ZoomDirection::In);
            }
            ClickMode::JuliaConstant => {
                state.render_settings.julia_constant = state.render_settings.coord_to_c(canvas_pos)
            }
            ClickMode::MandelConstant => {
                state.render_settings.mandel_constant = state.render_settings.coord_to_c(canvas_pos)
            }
            ClickMode::BailOut => {
                state.render_settings.bailout = state
                    .render_settings
                    .coord_to_c(canvas_pos)
                    .abs()
                    .real()
                    .to_f32()
            }
            ClickMode::Info => {
                let point = state.render_settings.coord_to_c(canvas_pos);
                state.log_info_title(
                    "Click Info",
                    format!(
                        "Real: <acc {}>\nImag: <acc {}>\nDiverg: <acc {}>",
                        point.real().to_f32(),
                        point.imag().to_f32(),
                        (state.render_settings.get_frac_clos())(point, &state.render_settings),
                    ),
                )
            }
        }

        state.request_redraw();
    }
}
