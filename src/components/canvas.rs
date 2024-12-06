//! Contains the `Canvas` widget.

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
    layout::{Alignment, Rect},
    style::Style,
    symbols::Marker,
    text::Line,
    widgets::{canvas::Points, Block, Widget},
};
use rug::Float;
use std::ops::{AddAssign, SubAssign};

use crate::{
    app::CanvasPoints,
    app_state::ClickMode,
    colors,
    fractals::FRACTALS,
    helpers::{void_fills, Focus, ZoomDirection},
    AppState,
};

pub(crate) struct Canvas<'a> {
    state: &'a AppState,
    points: &'a CanvasPoints,
}

impl<'a> Canvas<'a> {
    pub(crate) const FOOTER_TEXT: &'static [&'static str] = &[
        "Move[arrow/Vim keys]",
        "Zoom-[s]",
        "Zoom+[d]",
        "PalOffset+[+]",
        "PalOffset-[-]",
        "MxDiv-[y]",
        "Prec-[u]",
        "Prec+[i]",
        "MxDiv+[o]",
        "Color[c]",
        "Frac[f]",
        "VoidFill[v]",
        "Rst[r]",
    ];
    pub(crate) fn new(state: &'a AppState, points: &'a CanvasPoints) -> Self {
        Self { state, points }
    }

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
                state.render_settings.decrement_color_offset();
                state.request_repaint();
            }
            // Increment color scheme offset
            KeyCode::Char('+') => {
                state.render_settings.increment_color_offset();
                state.request_repaint();
            }
            // Cycle through the void fills
            KeyCode::Char('v') => {
                state.render_settings.void_fill_index =
                    (state.render_settings.void_fill_index + 1) % void_fills().len();
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

impl Widget for Canvas<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We need a ternary operator pleasssssse
        let border_style = Style::default().fg(if self.state.focused == Focus::Canvas {
            ratatui::style::Color::LightBlue
        } else {
            ratatui::style::Color::DarkGray
        });

        let canvas_block = Block::bordered()
            .style(border_style)
            .title_bottom(
                Line::from(format!(
                    "VoidFill[{}]",
                    void_fills()[self.state.render_settings.void_fill_index]
                ))
                .right_aligned(),
            )
            .title_bottom(
                Line::from(format!("Pts[{}]", self.state.render_settings.point_count()))
                    .left_aligned(),
            )
            .title_bottom(
                Line::from(format!("AvgDiv[{:.2}]", self.state.stats.avg_diverg)).left_aligned(),
            )
            .title_bottom(
                Line::from(format!(
                    "Colors[{}+{}]",
                    self.state.render_settings.get_palette().name,
                    self.state.render_settings.color_scheme_offset
                ))
                .right_aligned(),
            )
            .title_bottom(
                Line::from(format!("HighDiv[{}]", self.state.stats.highest_diverg)).left_aligned(),
            )
            .title_top(
                Line::from(format!("MxDiv[{}]", self.state.render_settings.max_iter))
                    .right_aligned(),
            )
            .title_top(
                Line::from(format!(
                    "RndrTime[{}ms]",
                    self.state.stats.render_time.as_millis()
                ))
                .right_aligned(),
            )
            .title_top(
                Line::from(format!(
                    "{}[x{:.3e}]",
                    self.state.render_settings.get_frac_obj().name,
                    self.state.render_settings.get_zoom()
                ))
                .left_aligned()
                .style(Style::default().fg(ratatui::style::Color::LightGreen)),
            )
            .title_top(
                Line::from(format!(
                    "GpuMode[{}]",
                    if self.state.render_settings.use_gpu {
                        "on"
                    } else {
                        "off"
                    }
                ))
                .left_aligned(),
            )
            .title_top(
                Line::from(format!("Prec[{}]", self.state.render_settings.prec)).right_aligned(),
            )
            .title_style(Style::default().fg(ratatui::style::Color::White))
            .title_alignment(Alignment::Center);

        let canvas_wid = ratatui::widgets::canvas::Canvas::default()
            .marker(Marker::HalfBlock)
            .block(canvas_block)
            .x_bounds([0.0, self.state.render_settings.canvas_size.x as f64 - 1.0])
            // ___________________
            // |h;0           w;h|
            // |                 |
            // |                 |
            // |^                |
            // |O;0→          w;0|
            // -------------------
            .y_bounds([0.0, self.state.render_settings.canvas_size.y as f64 - 1.0])
            .paint(|ctx| {
                for (color, points) in self.points {
                    ctx.draw(&Points {
                        color: *color,
                        coords: points,
                    })
                }
            });

        canvas_wid.render(area, buf);
    }
}
