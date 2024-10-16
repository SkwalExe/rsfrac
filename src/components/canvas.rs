use std::fmt::format;
use std::ops::{AddAssign, SubAssign};

use crate::app::fractal_logic::ratatui_to_canvas_coords;
use crate::app::void_fills;
use crate::fractals::FRACTALS;
use crate::helpers::{Focus, ZoomDirection};
use crate::{app::App, colors};
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

pub struct Canvas<'a> {
    app: &'a App,
}

impl<'a> Canvas<'a> {
    pub const FOOTER_TEXT: &'static [&'static str] = &[
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
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn handle_mouse_event(app: &mut App, event: MouseEvent) {
        // If the canvas is not already focused, only take focus
        if app.focused != Focus::Canvas {
            app.focused = Focus::Canvas;
            return;
        }

        // If the canvas is already focused, handle the event
        // first, convert the key press position to canvas coordinates

        let canvas_pos = ratatui_to_canvas_coords(app, event.column, event.row);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                app.zoom_at(canvas_pos, ZoomDirection::In);
            }
            MouseEventKind::Down(MouseButton::Right) => {
                app.zoom_at(canvas_pos, ZoomDirection::Out);
            }
            _ => {}
        }

        app.redraw_canvas = true;
    }

    pub fn handle_key_code(app: &mut App, code: KeyCode) {
        match code {
            // When H is pressed move the position of the canvas
            // to the left by r times the cell size.
            KeyCode::Char('h') => {
                app.render_settings
                    .pos
                    .mut_real()
                    .sub_assign(Float::with_val(
                        app.render_settings.prec,
                        &app.render_settings.cell_size * app.move_dist,
                    ));
            }
            // When L is pressed move the position of the canvas
            // to the right by n times the cell size.
            KeyCode::Char('l') => {
                app.render_settings
                    .pos
                    .mut_real()
                    .add_assign(Float::with_val(
                        app.render_settings.prec,
                        &app.render_settings.cell_size * app.move_dist,
                    ));
            }
            // When J is pressed move the position of the canvas
            // down by n times the cell size.
            KeyCode::Char('j') => {
                app.render_settings
                    .pos
                    .mut_imag()
                    .sub_assign(Float::with_val(
                        app.render_settings.prec,
                        &app.render_settings.cell_size * app.move_dist,
                    ));
            }
            // When K is pressed move the position of the canvas
            // up by n times the cell size.
            KeyCode::Char('k') => {
                app.render_settings
                    .pos
                    .mut_imag()
                    .add_assign(Float::with_val(
                        app.render_settings.prec,
                        &app.render_settings.cell_size * app.move_dist,
                    ));
            }
            // When S is pressed increase the cell size, which will zoom out of the canvas
            KeyCode::Char('s') => app.zoom(ZoomDirection::Out),
            // When D is pressed decrease the cell size, which will zoom into the canvas
            KeyCode::Char('d') => app.zoom(ZoomDirection::In),
            // decrease the decimal precision
            KeyCode::Char('u') => {
                app.increment_decimal_prec(-10);
            }
            // increase the decimal precision
            KeyCode::Char('i') => {
                app.increment_decimal_prec(10);
            }
            // reset the position to the origin and the cell size.
            KeyCode::Char('r') => {
                app.reset_cell_size();
                app.reset_pos();
            }
            // Increment the selected frac index
            KeyCode::Char('f') => {
                app.render_settings.frac_index =
                    (app.render_settings.frac_index + 1) % FRACTALS.len();
            }
            // Increment the color palette index
            KeyCode::Char('c') => {
                app.palette_index = (app.palette_index + 1) % colors::COLORS.len();
            }
            // Todo: remove duplication for + and -
            // Increment color scheme offset
            KeyCode::Char('-') => {
                app.color_scheme_offset =
                    (app.color_scheme_offset + app.get_palette().colors.len() as i32 - 1)
                        % app.get_palette().colors.len() as i32
            }
            // Increment color scheme offset
            KeyCode::Char('+') => {
                app.color_scheme_offset =
                    (app.color_scheme_offset + 1) % app.get_palette().colors.len() as i32
            }
            // Cycle through the void fills
            KeyCode::Char('v') => {
                app.render_settings.void_fill_index =
                    (app.render_settings.void_fill_index + 1) % void_fills().len();
                app.log_info_title(
                    "Void Fill",
                    format!(
                        "Void fill is now: <acc {}>",
                        void_fills()[app.render_settings.void_fill_index]
                    ),
                )
            }
            // Increment the maximum divergence
            KeyCode::Char('o') => app.increment_max_iter(10),
            // Decrement the maximum divergence
            KeyCode::Char('y') => app.increment_max_iter(-10),
            _ => {
                // Return from the function to avoid setting redraw_canvas
                return;
            }
        }

        // For now, all events need to redraw the canvas.
        app.redraw_canvas = true;
    }
}
impl<'a> Widget for Canvas<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We need a ternary operator pleasssssse
        let border_style = Style::default().fg(if self.app.focused == Focus::Canvas {
            ratatui::style::Color::LightBlue
        } else {
            ratatui::style::Color::DarkGray
        });

        let canvas_block = Block::bordered()
            .style(border_style)
            .title_bottom(Line::from(format!("Pts[{}]", self.app.point_count())).left_aligned())
            .title_bottom(
                Line::from(format!("AvgDiv[{:.2}]", self.app.stats.avg_diverg)).left_aligned(),
            )
            .title_bottom(
                Line::from(format!("PalOffset[{}]", self.app.color_scheme_offset)).right_aligned(),
            )
            .title_bottom(
                Line::from(format!("Colors[{}]", self.app.get_palette().name)).right_aligned(),
            )
            .title_bottom(
                Line::from(format!("HighDiv[{}]", self.app.stats.highest_diverg)).left_aligned(),
            )
            .title_top(
                Line::from(format!("MxDiv[{}]", self.app.render_settings.max_iter)).right_aligned(),
            )
            .title_top(
                Line::from(format!(
                    "RndrTime[{}ms]",
                    self.app.stats.render_time.as_millis()
                ))
                .right_aligned(),
            )
            .title_top(
                Line::from(format!("Mandelbrot[x{:.3e}]", self.app.get_zoom()))
                    .left_aligned()
                    .style(Style::default().fg(ratatui::style::Color::LightGreen)),
            )
            .title_top(
                Line::from(format!("Prec[{}]", self.app.render_settings.prec)).right_aligned(),
            )
            .title_style(Style::default().fg(ratatui::style::Color::White))
            .title_alignment(Alignment::Center);

        let canvas_wid = ratatui::widgets::canvas::Canvas::default()
            .marker(Marker::HalfBlock)
            .block(canvas_block)
            .x_bounds([0.0, self.app.render_settings.canvas_size.x as f64 - 1.0])
            // ___________________
            // |h;0           w;h|
            // |                 |
            // |                 |
            // |^                |
            // |O;0→          w;0|
            // -------------------
            .y_bounds([0.0, self.app.render_settings.canvas_size.y as f64 - 1.0])
            .paint(|ctx| {
                for (color, points) in &self.app.points {
                    ctx.draw(&Points {
                        color: *color,
                        coords: points,
                    })
                }
                if let Some(marker) = &self.app.marker {
                    // Todo: use a Painter instead
                    ctx.draw(&Points {
                        color: Color::Rgb(255, 0, 0),
                        coords: &[(
                            (marker.x + self.app.render_settings.canvas_size.x / 2) as f64,
                            (marker.y + self.app.render_settings.canvas_size.y / 2) as f64,
                        )],
                    })
                }
            });

        canvas_wid.render(area, buf);
    }
}
