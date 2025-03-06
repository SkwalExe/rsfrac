use ratatui::crossterm::event::KeyCode;
use rug::Float;
use std::ops::{AddAssign, SubAssign};

use crate::{
    app_state::hsl_settings::MAX_HSL_VALUE,
    colors,
    fractals::FRACTALS,
    helpers::{decrement_wrap, increment_wrap, void_fills, ZoomDirection},
    AppState,
};

use super::{selectable_variables, Canvas, SelectedVariable};
impl Canvas<'_> {
    pub(crate) fn handle_key_code(state: &mut AppState, code: KeyCode) {
        match code {
            // When H is pressed move the position of the canvas
            // to the left by r times the cell size.
            KeyCode::Char('h') | KeyCode::Left => {
                state
                    .render_settings
                    .pos
                    .mut_real()
                    .sub_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
                state.request_redraw();
            }
            // When L is pressed move the position of the canvas
            // to the right by n times the cell size.
            KeyCode::Char('l') | KeyCode::Right => {
                state
                    .render_settings
                    .pos
                    .mut_real()
                    .add_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
                state.request_redraw();
            }
            // When J is pressed move the position of the canvas
            // down by n times the cell size.
            KeyCode::Char('j') | KeyCode::Down => {
                state
                    .render_settings
                    .pos
                    .mut_imag()
                    .sub_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
                state.request_redraw();
            }
            // When K is pressed move the position of the canvas
            // up by n times the cell size.
            KeyCode::Char('k') | KeyCode::Up => {
                state
                    .render_settings
                    .pos
                    .mut_imag()
                    .add_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
                state.request_redraw();
            }
            // When S is pressed increase the cell size, which will zoom out of the canvas
            KeyCode::Char('s') => {
                state.zoom(ZoomDirection::Out);

                // TODO: mueheheeh
                // state.render_settings.color_scheme_offset =
                //     (state.render_settings.color_scheme_offset + 15) % 16;
                //
                state.request_redraw();
            }
            // When D is pressed decrease the cell size, which will zoom into the canvas
            KeyCode::Char('d') => {
                state.zoom(ZoomDirection::In);

                // TODO: mueheheeh
                // state.render_settings.color_scheme_offset =
                //     (state.render_settings.color_scheme_offset + 1) % 16;

                state.request_redraw();
            }
            // decrease the decimal precision
            KeyCode::Char('u') => {
                state.increment_decimal_prec(-10);
                state.request_redraw();
            }
            // increase the decimal precision
            KeyCode::Char('i') => {
                state.increment_decimal_prec(10);
                state.request_redraw();
            }
            // reset the position to the origin and the cell size.
            KeyCode::Char('r') => {
                state.render_settings.reset_cell_size();
                state.render_settings.reset_pos();
                state.request_redraw();
            }
            // Increment the selected frac index
            KeyCode::Char('f') => {
                let frac_i = (state.render_settings.frac_index + 1) % FRACTALS.len();
                let res = state.render_settings.select_fractal(frac_i);
                state.handle_res(res);
                state.request_redraw();
            }
            // Increment the color palette index
            KeyCode::Char('c') => {
                state.render_settings.palette_index =
                    (state.render_settings.palette_index + 1) % colors::COLORS.len();
                state.request_repaint();
            }
            // Todo: remove duplication for + and -
            // Increment color scheme offset
            KeyCode::Char('-') => {
                match selectable_variables()[state.selected_canvas_variable] {
                    SelectedVariable::PaletteOffset => {
                        state.render_settings.decrement_color_offset()
                    }

                    SelectedVariable::HSLLum => {
                        decrement_wrap(&mut state.render_settings.hsl_settings.lum, MAX_HSL_VALUE)
                    }

                    SelectedVariable::HSLSat => decrement_wrap(
                        &mut state.render_settings.hsl_settings.saturation,
                        MAX_HSL_VALUE,
                    ),

                    SelectedVariable::HueOffset => decrement_wrap(
                        &mut state.render_settings.hsl_settings.hue_offset,
                        MAX_HSL_VALUE,
                    ),

                    SelectedVariable::HSLSmoothness => decrement_wrap(
                        &mut state.render_settings.hsl_settings.smoothness,
                        MAX_HSL_VALUE,
                    ),
                }
                state.request_repaint();
            }
            // Increment color scheme offset
            KeyCode::Char('+') => {
                match selectable_variables()[state.selected_canvas_variable] {
                    SelectedVariable::PaletteOffset => {
                        state.render_settings.increment_color_offset()
                    }
                    SelectedVariable::HSLLum => {
                        increment_wrap(&mut state.render_settings.hsl_settings.lum, MAX_HSL_VALUE)
                    }
                    SelectedVariable::HSLSat => increment_wrap(
                        &mut state.render_settings.hsl_settings.saturation,
                        MAX_HSL_VALUE,
                    ),
                    SelectedVariable::HSLSmoothness => increment_wrap(
                        &mut state.render_settings.hsl_settings.smoothness,
                        MAX_HSL_VALUE,
                    ),
                    SelectedVariable::HueOffset => increment_wrap(
                        &mut state.render_settings.hsl_settings.hue_offset,
                        MAX_HSL_VALUE,
                    ),
                }
                state.request_repaint();
            }
            // Toggle HSL mode
            KeyCode::Char('n') => {
                state.render_settings.hsl_settings.enabled =
                    !state.render_settings.hsl_settings.enabled;
                // Try to select another canvas var
                state.prevent_canvas_var_hidden();
                state.request_repaint();
            }
            // Cycle through the selectable variables
            KeyCode::Char('t') => state.next_canv_var(),
            // Cycle through the void fills
            KeyCode::Char('v') => {
                increment_wrap(
                    &mut state.render_settings.void_fill_index,
                    void_fills().len(),
                );
                state.request_repaint();
            }
            // Increment the maximum divergence
            KeyCode::Char('o') => state.increment_max_iter(10),
            // Decrement the maximum divergence
            KeyCode::Char('y') => state.increment_max_iter(-10),
            _ => {}
        }
    }
}
