//! Contains the logic state of the application (can be headless).

use std::{collections::HashMap, sync::Mutex};
use tui_input::Input as TuiInput;
use tui_scrollview::ScrollViewState;

mod click_modes;
pub(crate) mod default_app_state;
mod helpers;
pub(crate) mod hsl_settings;
mod state_loading;
mod stats;
pub(crate) use click_modes::{ClickConfig, ClickMode};
pub(crate) use stats::Stats;

use crate::{app::WaitingScreenshot, frac_logic::RenderSettings, helpers::Focus};

pub(crate) struct AppState {
    pub(crate) redraw_canvas: bool,
    pub(crate) repaint_canvas: bool,
    pub(crate) stats: Stats,
    pub(crate) focused: Focus,
    pub(crate) quit: bool,
    pub(crate) log_messages: Vec<String>,
    pub(crate) prioritized_log_messages: HashMap<i64, String>,
    pub(crate) log_panel_scroll_state: Mutex<ScrollViewState>,
    pub(crate) last_commands: Vec<String>,
    /// The i32 is where the user is at in the command history. -1 means he is entering a new
    /// command
    pub(crate) command_input: (TuiInput, i32),
    pub(crate) move_dist: i32,
    pub(crate) scaling_factor: i32,
    pub(crate) render_settings: RenderSettings,
    pub(crate) requested_jobs: Vec<WaitingScreenshot>,
    pub(crate) click_config: ClickConfig,
    pub(crate) remove_jobs: bool,
    pub(crate) pause_jobs: bool,
    /// The index, in selectable_variables() of the currently selected canvas variable
    pub(crate) selected_canvas_variable: usize,
}
