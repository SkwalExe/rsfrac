use crate::AppState;
use serde::{Deserialize, Serialize};

use super::{void_fills, VoidFill};

/// Describes the state data that can be saved to a rsf file.
#[derive(Serialize, Deserialize)]
pub(crate) struct SavedState {
    pub(crate) frac_name: Option<String>,
    pub(crate) color_palette_name: Option<String>,
    pub(crate) palette_offset: Option<i32>,
    pub(crate) pos: Option<String>,
    pub(crate) complex_width: Option<String>,
    pub(crate) precision: Option<u32>,
    pub(crate) max_iter: Option<i32>,
    pub(crate) void_fill: Option<VoidFill>,
    pub(crate) julia_constant: Option<String>,
    pub(crate) mandel_constant: Option<String>,
}

impl From<&AppState> for SavedState {
    fn from(state: &AppState) -> Self {
        Self {
            frac_name: Some(state.render_settings.get_frac_obj().name.to_string()),
            color_palette_name: Some(state.render_settings.get_palette().name.to_string()),
            palette_offset: Some(state.render_settings.color_scheme_offset),
            pos: Some(state.render_settings.pos.to_string()),
            complex_width: Some(state.render_settings.get_plane_wid().to_string()),
            max_iter: Some(state.render_settings.max_iter),
            precision: Some(state.render_settings.prec),
            void_fill: Some(void_fills()[state.render_settings.void_fill_index].clone()),
            julia_constant: Some(state.render_settings.julia_constant.to_string()),
            mandel_constant: Some(state.render_settings.mandel_constant.to_string()),
        }
    }
}
