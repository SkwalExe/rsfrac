use ratatui::crossterm::event::KeyCode;
use rug::Float;
use std::ops::{AddAssign, SubAssign};

use crate::{
    app_state::hsl_settings::MAX_HSL_VALUE,
    colors,
    fractals::FRACTALS,
    helpers::{decrement_wrap, increment_wrap, void_fills, ZoomDirection},
    App,
};

use super::{selectable_variables, Canvas, SelectedVariable};
impl Canvas<'_> {
    pub(crate) fn handle_key_code(app: &mut App, code: KeyCode) {
        match code {
            // When H is pressed move the position of the canvas
            // to the left by r times the cell size.
            KeyCode::Char('h') | KeyCode::Left => {
                app.app_state
                    .render_settings
                    .pos
                    .mut_real()
                    .sub_assign(Float::with_val(
                        app.app_state.render_settings.prec,
                        &app.app_state.render_settings.cell_size * app.app_state.move_dist,
                    ));
                app.app_state.request_redraw();
            }
            // When L is pressed move the position of the canvas
            // to the right by n times the cell size.
            KeyCode::Char('l') | KeyCode::Right => {
                app.app_state
                    .render_settings
                    .pos
                    .mut_real()
                    .add_assign(Float::with_val(
                        app.app_state.render_settings.prec,
                        &app.app_state.render_settings.cell_size * app.app_state.move_dist,
                    ));
                app.app_state.request_redraw();
            }
            // When J is pressed move the position of the canvas
            // down by n times the cell size.
            KeyCode::Char('j') | KeyCode::Down => {
                app.app_state
                    .render_settings
                    .pos
                    .mut_imag()
                    .sub_assign(Float::with_val(
                        app.app_state.render_settings.prec,
                        &app.app_state.render_settings.cell_size * app.app_state.move_dist,
                    ));
                app.app_state.request_redraw();
            }
            // When K is pressed move the position of the canvas
            // up by n times the cell size.
            KeyCode::Char('k') | KeyCode::Up => {
                app.app_state
                    .render_settings
                    .pos
                    .mut_imag()
                    .add_assign(Float::with_val(
                        app.app_state.render_settings.prec,
                        &app.app_state.render_settings.cell_size * app.app_state.move_dist,
                    ));
                app.app_state.request_redraw();
            }
            // When S is pressed increase the cell size, which will zoom out of the canvas
            KeyCode::Char('s') => {
                app.app_state.zoom(ZoomDirection::Out);

                // TODO: mueheheeh
                // app.app_state.render_settings.color_scheme_offset =
                //     (app.app_state.render_settings.color_scheme_offset + 15) % 16;
                //
                app.app_state.request_redraw();
            }
            // When b is pressed toggle the side panel
            KeyCode::Char('b') => app.toggle_sidepanel(),
            // When D is pressed decrease the cell size, which will zoom into the canvas
            KeyCode::Char('d') => {
                app.app_state.zoom(ZoomDirection::In);

                // TODO: mueheheeh
                // app.app_state.render_settings.color_scheme_offset =
                //     (app.app_state.render_settings.color_scheme_offset + 1) % 16;

                app.app_state.request_redraw();
            }
            // decrease the decimal precision
            KeyCode::Char('u') => {
                app.app_state.increment_decimal_prec(-10);
                app.app_state.request_redraw();
            }
            // increase the decimal precision
            KeyCode::Char('i') => {
                app.app_state.increment_decimal_prec(10);
                app.app_state.request_redraw();
            }
            // reset the position to the origin and the cell size.
            KeyCode::Char('r') => {
                app.app_state.render_settings.reset_cell_size();
                app.app_state.render_settings.reset_pos();
                app.app_state.request_redraw();
            }
            // Increment the selected frac index
            KeyCode::Char('f') => {
                let frac_i = (app.app_state.render_settings.frac_index + 1) % FRACTALS.len();
                let res = app.app_state.render_settings.select_fractal(frac_i);
                app.app_state.handle_res(res);
                app.app_state.request_redraw();
            }
            // Increment the color palette index
            KeyCode::Char('c') => {
                app.app_state.render_settings.palette_index =
                    (app.app_state.render_settings.palette_index + 1) % colors::COLORS.len();
                app.app_state.request_repaint();
            }
            // Todo: remove duplication for + and -
            // Increment color scheme offset
            KeyCode::Char('-') => {
                match selectable_variables()[app.app_state.selected_canvas_variable] {
                    SelectedVariable::PaletteOffset => {
                        app.app_state.render_settings.decrement_color_offset()
                    }

                    SelectedVariable::HSLLum => decrement_wrap(
                        &mut app.app_state.render_settings.hsl_settings.lum,
                        MAX_HSL_VALUE,
                    ),

                    SelectedVariable::HSLSat => decrement_wrap(
                        &mut app.app_state.render_settings.hsl_settings.saturation,
                        MAX_HSL_VALUE,
                    ),

                    SelectedVariable::HueOffset => decrement_wrap(
                        &mut app.app_state.render_settings.hsl_settings.hue_offset,
                        MAX_HSL_VALUE,
                    ),

                    SelectedVariable::HSLSmoothness => decrement_wrap(
                        &mut app.app_state.render_settings.hsl_settings.smoothness,
                        MAX_HSL_VALUE,
                    ),
                }
                app.app_state.request_repaint();
            }
            // Increment color scheme offset
            KeyCode::Char('+') => {
                match selectable_variables()[app.app_state.selected_canvas_variable] {
                    SelectedVariable::PaletteOffset => {
                        app.app_state.render_settings.increment_color_offset()
                    }
                    SelectedVariable::HSLLum => increment_wrap(
                        &mut app.app_state.render_settings.hsl_settings.lum,
                        MAX_HSL_VALUE,
                    ),
                    SelectedVariable::HSLSat => increment_wrap(
                        &mut app.app_state.render_settings.hsl_settings.saturation,
                        MAX_HSL_VALUE,
                    ),
                    SelectedVariable::HSLSmoothness => increment_wrap(
                        &mut app.app_state.render_settings.hsl_settings.smoothness,
                        MAX_HSL_VALUE,
                    ),
                    SelectedVariable::HueOffset => increment_wrap(
                        &mut app.app_state.render_settings.hsl_settings.hue_offset,
                        MAX_HSL_VALUE,
                    ),
                }
                app.app_state.request_repaint();
            }
            // Toggle HSL mode
            KeyCode::Char('n') => {
                app.app_state.render_settings.hsl_settings.enabled =
                    !app.app_state.render_settings.hsl_settings.enabled;
                // Try to select another canvas var
                app.app_state.prevent_canvas_var_hidden();
                app.app_state.request_repaint();
            }
            // Cycle through the selectable variables
            KeyCode::Char('t') => app.app_state.next_canv_var(),
            // Cycle through the void fills
            KeyCode::Char('v') => {
                increment_wrap(
                    &mut app.app_state.render_settings.void_fill_index,
                    void_fills().len(),
                );
                app.app_state.request_repaint();
            }
            // Increment the maximum divergence
            KeyCode::Char('o') => app.app_state.increment_max_iter(10),
            // Decrement the maximum divergence
            KeyCode::Char('y') => app.app_state.increment_max_iter(-10),
            _ => {}
        }
    }
}
