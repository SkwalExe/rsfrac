use std::ops::{AddAssign, SubAssign};

use crate::app::{void_fills, AppState, CanvasPoints};
use crate::colors;
use crate::fractals::FRACTALS;
use crate::helpers::{Focus, ZoomDirection};
use ratatui::crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::text::Line;
use ratatui::widgets::canvas::Points;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};
use rug::Float;

pub(crate) struct Canvas<'a> {
    app_state: &'a AppState,
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
    pub(crate) fn new(app_state: &'a AppState, points: &'a CanvasPoints) -> Self {
        Self { app_state, points }
    }

    pub(crate) fn handle_mouse_event(app_state: &mut AppState, event: MouseEvent) {
        // If the canvas is not already focused, only take focus
        if app_state.focused != Focus::Canvas {
            app_state.focused = Focus::Canvas;
            return;
        }

        // If the canvas is already focused, handle the event
        // first, convert the key press position to canvas coordinates

        let canvas_pos = app_state
            .render_settings
            .ratatui_to_canvas_coords(event.column, event.row);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                app_state.zoom_at(canvas_pos, ZoomDirection::In);
            }
            MouseEventKind::Down(MouseButton::Right) => {
                app_state.zoom_at(canvas_pos, ZoomDirection::Out);
            }
            _ => {}
        }

        app_state.redraw_canvas = true;
    }

    pub(crate) fn handle_key_code(app_state: &mut AppState, code: KeyCode) {
        match code {
            // When H is pressed move the position of the canvas
            // to the left by r times the cell size.
            KeyCode::Char('h') => {
                app_state
                    .render_settings
                    .pos
                    .mut_real()
                    .sub_assign(Float::with_val(
                        app_state.render_settings.prec,
                        &app_state.render_settings.cell_size * app_state.move_dist,
                    ));
            }
            // When L is pressed move the position of the canvas
            // to the right by n times the cell size.
            KeyCode::Char('l') => {
                app_state
                    .render_settings
                    .pos
                    .mut_real()
                    .add_assign(Float::with_val(
                        app_state.render_settings.prec,
                        &app_state.render_settings.cell_size * app_state.move_dist,
                    ));
            }
            // When J is pressed move the position of the canvas
            // down by n times the cell size.
            KeyCode::Char('j') => {
                app_state
                    .render_settings
                    .pos
                    .mut_imag()
                    .sub_assign(Float::with_val(
                        app_state.render_settings.prec,
                        &app_state.render_settings.cell_size * app_state.move_dist,
                    ));
            }
            // When K is pressed move the position of the canvas
            // up by n times the cell size.
            KeyCode::Char('k') => {
                app_state
                    .render_settings
                    .pos
                    .mut_imag()
                    .add_assign(Float::with_val(
                        app_state.render_settings.prec,
                        &app_state.render_settings.cell_size * app_state.move_dist,
                    ));
            }
            // When S is pressed increase the cell size, which will zoom out of the canvas
            KeyCode::Char('s') => app_state.zoom(ZoomDirection::Out),
            // When D is pressed decrease the cell size, which will zoom into the canvas
            KeyCode::Char('d') => app_state.zoom(ZoomDirection::In),
            // decrease the decimal precision
            KeyCode::Char('u') => {
                app_state.increment_decimal_prec(-10);
            }
            // increase the decimal precision
            KeyCode::Char('i') => {
                app_state.increment_decimal_prec(10);
            }
            // reset the position to the origin and the cell size.
            KeyCode::Char('r') => {
                app_state.render_settings.reset_cell_size();
                app_state.render_settings.reset_pos();
            }
            // Increment the selected frac index
            KeyCode::Char('f') => {
                app_state.render_settings.frac_index =
                    (app_state.render_settings.frac_index + 1) % FRACTALS.len();
            }
            // Increment the color palette index
            KeyCode::Char('c') => {
                app_state.palette_index = (app_state.palette_index + 1) % colors::COLORS.len();
            }
            // Todo: remove duplication for + and -
            // Increment color scheme offset
            KeyCode::Char('-') => {
                app_state.color_scheme_offset = (app_state.color_scheme_offset
                    + app_state.get_palette().colors.len() as i32
                    - 1)
                    % app_state.get_palette().colors.len() as i32
            }
            // Increment color scheme offset
            KeyCode::Char('+') => {
                app_state.color_scheme_offset = (app_state.color_scheme_offset + 1)
                    % app_state.get_palette().colors.len() as i32
            }
            // Cycle through the void fills
            KeyCode::Char('v') => {
                app_state.void_fill_index = (app_state.void_fill_index + 1) % void_fills().len();
                app_state.log_info_title(
                    "Void Fill",
                    format!(
                        "Void fill is now: <acc {}>",
                        void_fills()[app_state.void_fill_index]
                    ),
                )
            }
            // Increment the maximum divergence
            KeyCode::Char('o') => app_state.increment_max_iter(10),
            // Decrement the maximum divergence
            KeyCode::Char('y') => app_state.increment_max_iter(-10),
            _ => {
                // Return from the function to avoid setting redraw_canvas
                return;
            }
        }

        // For now, all events need to redraw the canvas.
        app_state.redraw_canvas = true;
    }
}
impl<'a> Widget for Canvas<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We need a ternary operator pleasssssse
        let border_style = Style::default().fg(if self.app_state.focused == Focus::Canvas {
            ratatui::style::Color::LightBlue
        } else {
            ratatui::style::Color::DarkGray
        });

        let canvas_block = Block::bordered()
            .style(border_style)
            .title_bottom(
                Line::from(format!(
                    "Pts[{}]",
                    self.app_state.render_settings.point_count()
                ))
                .left_aligned(),
            )
            .title_bottom(
                Line::from(format!("AvgDiv[{:.2}]", self.app_state.stats.avg_diverg))
                    .left_aligned(),
            )
            .title_bottom(
                Line::from(format!("PalOffset[{}]", self.app_state.color_scheme_offset))
                    .right_aligned(),
            )
            .title_bottom(
                Line::from(format!("Colors[{}]", self.app_state.get_palette().name))
                    .right_aligned(),
            )
            .title_bottom(
                Line::from(format!("HighDiv[{}]", self.app_state.stats.highest_diverg))
                    .left_aligned(),
            )
            .title_top(
                Line::from(format!(
                    "MxDiv[{}]",
                    self.app_state.render_settings.max_iter
                ))
                .right_aligned(),
            )
            .title_top(
                Line::from(format!(
                    "RndrTime[{}ms]",
                    self.app_state.stats.render_time.as_millis()
                ))
                .right_aligned(),
            )
            .title_top(
                Line::from(format!(
                    "Mandelbrot[x{:.3e}]",
                    self.app_state.render_settings.get_zoom()
                ))
                .left_aligned()
                .style(Style::default().fg(ratatui::style::Color::LightGreen)),
            )
            .title_top(
                Line::from(format!("Prec[{}]", self.app_state.render_settings.prec))
                    .right_aligned(),
            )
            .title_style(Style::default().fg(ratatui::style::Color::White))
            .title_alignment(Alignment::Center);

        let canvas_wid = ratatui::widgets::canvas::Canvas::default()
            .marker(Marker::HalfBlock)
            .block(canvas_block)
            .x_bounds([
                0.0,
                self.app_state.render_settings.canvas_size.x as f64 - 1.0,
            ])
            // ___________________
            // |h;0           w;h|
            // |                 |
            // |                 |
            // |^                |
            // |O;0→          w;0|
            // -------------------
            .y_bounds([
                0.0,
                self.app_state.render_settings.canvas_size.y as f64 - 1.0,
            ])
            .paint(|ctx| {
                for (color, points) in self.points {
                    ctx.draw(&Points {
                        color: *color,
                        coords: points,
                    })
                }
                if let Some(marker) = &self.app_state.marker {
                    // Todo: use a Painter instead
                    ctx.draw(&Points {
                        color: Color::Rgb(255, 0, 0),
                        coords: &[(
                            (marker.x + self.app_state.render_settings.canvas_size.x / 2) as f64,
                            (marker.y + self.app_state.render_settings.canvas_size.y / 2) as f64,
                        )],
                    })
                }
            });

        canvas_wid.render(area, buf);
    }
}
