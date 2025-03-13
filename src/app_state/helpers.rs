use wgpu::Backends;

use crate::{
    components::{
        canvas::{selectable_variables, Canvas, SelectedVariable},
        Input, LogPanel,
    },
    frac_logic::CanvasCoords,
    helpers::{Focus, ZoomDirection},
};

use super::{
    default_app_state::{DF_MOVE_DISTANCE_CPU, DF_SCALING_FACTOR_CPU},
    AppState,
};
impl AppState {
    /// List available WGPU adapters and put them in .detected_adapters.
    /// Do nothing if gpu mode is disabled.
    pub(crate) fn detect_adapters(&mut self) {
        // Do nothing if GPU mode is disabled, because we unwrap below...
        if !self.render_settings.wgpu_state.use_gpu {
            return;
        }

        self.detected_adapters = self
            .render_settings
            .wgpu_state
            .instance
            .as_ref()
            .unwrap()
            .enumerate_adapters(Backends::all());
    }

    /// Select the next canvas var no matter if it will be hidden or not
    /// (for example when an HSL parameter is selected but hsl mode is off)
    pub(crate) fn next_canv_var_(&mut self) {
        self.selected_canvas_variable =
            (self.selected_canvas_variable + 1) % selectable_variables().len();
    }
    /// Returns true if the currently selected canvas var is hidden
    pub(crate) fn is_selected_var_hidden(&self) -> bool {
        if !self.render_settings.hsl_settings.enabled {
            // If hsl mode is not enabled
            self.is_var_selected(SelectedVariable::HSLSat)
                || self.is_var_selected(SelectedVariable::HSLLum)
                || self.is_var_selected(SelectedVariable::HueOffset)
                || self.is_var_selected(SelectedVariable::HSLSmoothness)
        } else if self.render_settings.hsl_settings.enabled {
            // if hsl mode is enabled
            self.is_var_selected(SelectedVariable::PaletteOffset)
        } else {
            false
        }
    }
    /// Loop canvas parameters until the selected one is visible
    pub(crate) fn prevent_canvas_var_hidden(&mut self) {
        // This is an interesting way to mimic a do while...
        while {
            // eprintln!(
            //     "Currenly selected: {}",
            //     selectable_variables()[self.selected_canvas_variable]
            // );
            // Return true (change the selected variable again)
            // if the selected variable is not visible on the panel
            self.is_selected_var_hidden()
        } {
            // eprintln!("Selecting next available var.");
            self.next_canv_var_();
        }
    }
    /// Select the next visible canvas variable
    pub(crate) fn next_canv_var(&mut self) {
        self.next_canv_var_();
        self.prevent_canvas_var_hidden();
    }

    /// Returns true if the specified variable is selected in the canvas
    pub(crate) fn is_var_selected(&self, var: SelectedVariable) -> bool {
        // eprintln!(
        //     "Get variant: {var}, selected index is {}. List is {:?} -> {}",
        //     self.selected_canvas_variable,
        //     selectable_variables(),
        //     var == self.selected_canvas_variable
        // );
        var == self.selected_canvas_variable
    }
    /// Returns the last executed command or an empty string
    pub(crate) fn get_command(&self, index: i32) -> String {
        if index < 0 {
            return String::new();
        }
        self.last_commands
            .get(index as usize)
            .cloned()
            .unwrap_or_default()
    }

    /// Load default settings for CPU mode when GPU init fails at startup
    pub(crate) fn cpu_defaults(&mut self) {
        self.move_dist = DF_MOVE_DISTANCE_CPU;
        self.scaling_factor = DF_SCALING_FACTOR_CPU;
        self.render_settings.cpu_defaults();
    }

    /// Only repaint the canvas without generating a new divergence matrix.
    pub(crate) fn request_repaint(&mut self) {
        self.repaint_canvas = true;
    }
    /// Update the divergence matrix and repaint the canvas.
    pub(crate) fn request_redraw(&mut self) {
        self.redraw_canvas = true;
        self.request_repaint();
    }
    /// Return the text to display in the footer
    pub(crate) fn footer_text(&self) -> &'static [&'static str] {
        match self.focused {
            Focus::LogPanel => LogPanel::FOOTER_TEXT,
            Focus::Canvas => Canvas::FOOTER_TEXT,
            Focus::Input => Input::FOOTER_TEXT,
        }
    }
    /// Increment positively or negatively the maximum divergence, and ask for canvas redraw
    pub(crate) fn increment_max_iter(&mut self, increment: i32) {
        self.render_settings.increment_max_iter(increment);
        self.request_redraw();
    }
    /// Increment positively or negatively the decimal precision,
    /// and update the precision of existing numeric values.
    pub(crate) fn increment_decimal_prec(&mut self, increment: i32) {
        let new_prec = self.render_settings.prec.saturating_add_signed(increment);
        self.set_decimal_prec(new_prec);
    }

    /// Sets the decimal precision and update the precision of existing values.
    pub(crate) fn set_decimal_prec(&mut self, prec: u32) {
        self.render_settings.set_decimal_prec(prec);
        // Ask for canvas redraw
        self.request_redraw();
    }
    pub(crate) fn zoom_at(&mut self, pos: CanvasCoords, direction: ZoomDirection) {
        let inintial_c_pos = self.render_settings.coord_to_c(pos.clone());
        self.zoom(direction);
        let new_c_pos = self.render_settings.coord_to_c(pos);

        self.render_settings.pos += inintial_c_pos - new_c_pos;
    }
    pub(crate) fn zoom(&mut self, direction: ZoomDirection) {
        let scaling_factor = 1.0 + self.scaling_factor as f64 / 100.0;

        match direction {
            ZoomDirection::In => self.render_settings.cell_size /= scaling_factor,
            ZoomDirection::Out => self.render_settings.cell_size *= scaling_factor,
        }
    }
}
