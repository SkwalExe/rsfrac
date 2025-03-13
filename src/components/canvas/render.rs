use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    symbols::Marker,
    text::{Line, Span},
    widgets::{canvas::Points, Block, Widget},
};

use crate::helpers::{void_fills, Focus};

use super::{Canvas, SelectedVariable};

fn red_if(st: impl Into<String>, cond: bool) -> Span<'static> {
    let as_string = st.into();
    // eprintln!("Coloring {as_string} to red: {cond}");
    if !cond {
        return Span::raw(as_string);
    }

    Span::styled(
        as_string,
        Style::default()
            .bg(ratatui::style::Color::Rgb(255, 90, 90))
            .fg(ratatui::style::Color::Rgb(0, 0, 0)),
    )
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
                Line::from(if self.state.render_settings.hsl_settings.enabled {
                    Vec::from([
                        "HSLMode[".into(),
                        red_if(
                            format!("{}", self.state.render_settings.hsl_settings.hue_offset),
                            self.state.is_var_selected(SelectedVariable::HueOffset),
                        ),
                        ",".into(),
                        red_if(
                            format!("{}", self.state.render_settings.hsl_settings.saturation),
                            self.state.is_var_selected(SelectedVariable::HSLSat),
                        ),
                        ",".into(),
                        red_if(
                            format!("{}", self.state.render_settings.hsl_settings.lum),
                            self.state.is_var_selected(SelectedVariable::HSLLum),
                        ),
                        ",smtss:".into(),
                        red_if(
                            format!("{}", self.state.render_settings.hsl_settings.smoothness),
                            self.state.is_var_selected(SelectedVariable::HSLSmoothness),
                        ),
                        "]".into(),
                    ])
                } else {
                    Vec::from([
                        "Palette[".into(),
                        self.state.render_settings.get_palette().name.into(),
                        "+".into(),
                        red_if(
                            self.state.render_settings.color_scheme_offset.to_string(),
                            self.state.is_var_selected(SelectedVariable::PaletteOffset),
                        ),
                        "]".into(),
                    ])
                })
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
                    if self.state.render_settings.wgpu_state.use_gpu {
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
