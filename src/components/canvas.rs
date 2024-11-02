//! Contains the `Canvas` widget.

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::Marker,
    text::Line,
    widgets::{canvas::Points, Block, Widget},
};
use rug::Float;
use std::ops::{AddAssign, SubAssign};

use crate::{
    app::CanvasPoints,
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
        // If the canvas is not already focused, only take focus
        if state.focused != Focus::Canvas {
            state.focused = Focus::Canvas;
            return;
        }

        // If the canvas is already focused, handle the event
        // first, convert the key press position to canvas coordinates

        let canvas_pos = state
            .render_settings
            .ratatui_to_canvas_coords(event.column, event.row);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                state.zoom_at(canvas_pos, ZoomDirection::In);
            }
            MouseEventKind::Down(MouseButton::Right) => {
                state.zoom_at(canvas_pos, ZoomDirection::Out);
            }
            _ => {}
        }

        state.redraw_canvas = true;
    }

    pub(crate) fn handle_key_code(state: &mut AppState, code: KeyCode) {
        match code {
            // When H is pressed move the position of the canvas
            // to the left by r times the cell size.
            KeyCode::Char('h') => {
                state
                    .render_settings
                    .pos
                    .mut_real()
                    .sub_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
            }
            // When L is pressed move the position of the canvas
            // to the right by n times the cell size.
            KeyCode::Char('l') => {
                state
                    .render_settings
                    .pos
                    .mut_real()
                    .add_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
            }
            // When J is pressed move the position of the canvas
            // down by n times the cell size.
            KeyCode::Char('j') => {
                state
                    .render_settings
                    .pos
                    .mut_imag()
                    .sub_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
            }
            // When K is pressed move the position of the canvas
            // up by n times the cell size.
            KeyCode::Char('k') => {
                state
                    .render_settings
                    .pos
                    .mut_imag()
                    .add_assign(Float::with_val(
                        state.render_settings.prec,
                        &state.render_settings.cell_size * state.move_dist,
                    ));
            }
            // When S is pressed increase the cell size, which will zoom out of the canvas
            KeyCode::Char('s') => state.zoom(ZoomDirection::Out),
            // When D is pressed decrease the cell size, which will zoom into the canvas
            KeyCode::Char('d') => state.zoom(ZoomDirection::In),
            // decrease the decimal precision
            KeyCode::Char('u') => {
                state.increment_decimal_prec(-10);
            }
            // increase the decimal precision
            KeyCode::Char('i') => {
                state.increment_decimal_prec(10);
            }
            // reset the position to the origin and the cell size.
            KeyCode::Char('r') => {
                state.render_settings.reset_cell_size();
                state.render_settings.reset_pos();
            }
            // Increment the selected frac index
            KeyCode::Char('f') => {
                state.render_settings.frac_index =
                    (state.render_settings.frac_index + 1) % FRACTALS.len();
            }
            // Increment the color palette index
            KeyCode::Char('c') => {
                state.palette_index = (state.palette_index + 1) % colors::COLORS.len();
            }
            // Todo: remove duplication for + and -
            // Increment color scheme offset
            KeyCode::Char('-') => {
                state.color_scheme_offset =
                    (state.color_scheme_offset + state.get_palette().colors.len() as i32 - 1)
                        % state.get_palette().colors.len() as i32
            }
            // Increment color scheme offset
            KeyCode::Char('+') => {
                state.color_scheme_offset =
                    (state.color_scheme_offset + 1) % state.get_palette().colors.len() as i32
            }
            // Cycle through the void fills
            KeyCode::Char('v') => {
                state.void_fill_index = (state.void_fill_index + 1) % void_fills().len();
                state.log_info_title(
                    "Void Fill",
                    format!(
                        "Void fill is now: <acc {}>",
                        void_fills()[state.void_fill_index]
                    ),
                )
            }
            // Increment the maximum divergence
            KeyCode::Char('o') => state.increment_max_iter(10),
            // Decrement the maximum divergence
            KeyCode::Char('y') => state.increment_max_iter(-10),
            _ => {
                // Return from the function to avoid setting redraw_canvas
                return;
            }
        }

        // For now, all events need to redraw the canvas.
        state.redraw_canvas = true;
    }
}
impl<'a> Widget for Canvas<'a> {
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
                Line::from(format!("Pts[{}]", self.state.render_settings.point_count()))
                    .left_aligned(),
            )
            .title_bottom(
                Line::from(format!("AvgDiv[{:.2}]", self.state.stats.avg_diverg)).left_aligned(),
            )
            .title_bottom(
                Line::from(format!("PalOffset[{}]", self.state.color_scheme_offset))
                    .right_aligned(),
            )
            .title_bottom(
                Line::from(format!("Colors[{}]", self.state.get_palette().name)).right_aligned(),
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
                    "Mandelbrot[x{:.3e}]",
                    self.state.render_settings.get_zoom()
                ))
                .left_aligned()
                .style(Style::default().fg(ratatui::style::Color::LightGreen)),
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
                if let Some(marker) = &self.state.marker {
                    // Todo: use a Painter instead
                    ctx.draw(&Points {
                        color: Color::Rgb(255, 0, 0),
                        coords: &[(
                            (marker.x + self.state.render_settings.canvas_size.x / 2) as f64,
                            (marker.y + self.state.render_settings.canvas_size.y / 2) as f64,
                        )],
                    })
                }
            });

        canvas_wid.render(area, buf);
    }
}
