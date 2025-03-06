use crate::{frac_logic::RenderSettings, VERSION};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, str::FromStr};

use super::{markup::esc, void_fills, VoidFill};

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
    pub(crate) hsl_mode: Option<bool>,
    pub(crate) hsl_saturation: Option<i32>,
    pub(crate) hsl_lum: Option<i32>,
    pub(crate) hsl_hue_offset: Option<i32>,
    pub(crate) hsl_smoothness: Option<i32>,
    pub(crate) version: Option<String>,
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
            hsl_lum: Some(rs.hsl_settings.lum),
            hsl_mode: Some(rs.hsl_settings.enabled),
            hsl_saturation: Some(rs.hsl_settings.saturation),
            hsl_hue_offset: Some(rs.hsl_settings.hue_offset),
            hsl_smoothness: Some(rs.hsl_settings.smoothness),
            version: Some(VERSION.to_string()),
        }
    }
}

impl FromStr for SavedState {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
            .map_err(|err| format!("Could not parse the provided state file: {}", esc(err)))
    }
}

impl RenderSettings {
    /// Saves the app state to an rsf file with the provided filename (extension included).
    pub(crate) fn save(&self, filename: &str) -> Result<(), String> {
        let saved_state = SavedState::from(self);
        let str = toml::to_string_pretty(&saved_state)
            .map_err(|err| format!("Could not save the current state: {}", esc(err)))?;

        let mut file = File::create(filename)
            .map_err(|err| format!("Could not create <command {filename}>: {}", esc(err)))?;

        file.write(str.as_bytes())
            .map_err(|err| format!("Could not write file: {}", esc(err)))?;
        Ok(())
    }
}
