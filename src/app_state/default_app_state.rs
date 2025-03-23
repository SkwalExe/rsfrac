use crate::components::canvas::{selectable_variables, SelectedVariable};

use super::AppState;

pub(crate) const DF_MOVE_DISTANCE_CPU: i32 = 8;
pub(crate) const DF_SCALING_FACTOR_CPU: i32 = 20;
pub(crate) const DF_MOVE_DISTANCE_GPU: i32 = 4;
pub(crate) const DF_SCALING_FACTOR_GPU: i32 = 8;

impl Default for AppState {
    fn default() -> Self {
        Self {
            stats: Default::default(),
            redraw_canvas: true,
            remove_jobs: false,
            repaint_canvas: true,
            last_commands: vec![],
            quit: false,
            focused: Default::default(),
            command_input: Default::default(),
            log_messages: Default::default(),
            prioritized_log_messages: Default::default(),
            log_panel_scroll_state: Default::default(),
            render_settings: Default::default(),
            scaling_factor: DF_SCALING_FACTOR_GPU,
            move_dist: DF_MOVE_DISTANCE_GPU,
            // Basically, since this is an index, we need to get the index of the default
            // value, which is PaletteOffset.
            selected_canvas_variable: selectable_variables()
                .iter()
                .position(|x| x.eq(&SelectedVariable::PaletteOffset))
                .unwrap(),
            requested_jobs: Default::default(),
            click_config: Default::default(),
            pause_jobs: false,
            detected_state_files: Default::default(),
            current_state_file_index: Default::default(),
        }
    }
}
