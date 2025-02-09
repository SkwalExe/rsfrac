use crate::frac_logic::RenderSettings;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, str::FromStr};

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
    pub(crate) bailout: Option<f32>,
    pub(crate) smoothness: Option<i32>,
}

impl From<&RenderSettings> for SavedState {
    fn from(rs: &RenderSettings) -> Self {
        Self {
            frac_name: Some(rs.get_frac_obj().name.to_string()),
            color_palette_name: Some(rs.get_palette().name.to_string()),
            palette_offset: Some(rs.color_scheme_offset),
            pos: Some(rs.pos.to_string()),
            complex_width: Some(rs.get_plane_wid().to_string()),
            max_iter: Some(rs.max_iter),
            precision: Some(rs.prec),
            void_fill: Some(void_fills()[rs.void_fill_index].clone()),
            julia_constant: Some(rs.julia_constant.to_string()),
            mandel_constant: Some(rs.mandel_constant.to_string()),
            bailout: Some(rs.bailout),
            smoothness: Some(rs.smoothness),
        }
    }
}

impl FromStr for SavedState {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
            .map_err(|err| format!("Could not parse the provided state file: {err}"))?
    }
}

impl RenderSettings {
    /// Saves the app state to an rsf file with the provided filename (extension included).
    pub(crate) fn save(&self, filename: &str) -> Result<(), String> {
        let saved_state = SavedState::from(self);
        let str = toml::to_string_pretty(&saved_state)
            .map_err(|err| format!("Could not save the current state: {err}"))?;

        let mut file = File::create(filename)
            .map_err(|err| format!("Could not create <command {filename}>: {err}"))?;

        file.write(str.as_bytes())
            .map_err(|err| format!("Could not write file: {err}"))?;
        Ok(())
    }
}
