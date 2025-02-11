//! Contains the ratatui and ansi markup generator logic.
// LATER: Find a way to remove the repetition here...

use std::fmt::Display;

use ansi_term::{Color as ANSIColor, Style as ANSIStyle};
use ratatui::style::{Color, Style};
use tui_markup::generator::{ANSIStringsGenerator, RatatuiTextGenerator};

const ACCENT_COLOR: MyOwnColorType = MyOwnColorType(162, 119, 255);
const RED: MyOwnColorType = MyOwnColorType(255, 109, 109);
const YELLOW: MyOwnColorType = MyOwnColorType(255, 183, 0);
const GREEN: MyOwnColorType = MyOwnColorType(88, 224, 178);
const DIM: MyOwnColorType = MyOwnColorType(42, 42, 42);
const COMMAND_BG: MyOwnColorType = MyOwnColorType(34, 40, 39);
const COMMAND_FG: MyOwnColorType = MyOwnColorType(217, 214, 207);
const BLACK: MyOwnColorType = MyOwnColorType(0, 0, 0);

struct MyOwnColorType(u8, u8, u8);

impl From<MyOwnColorType> for Color {
    fn from(value: MyOwnColorType) -> Self {
        Color::Rgb(value.0, value.1, value.2)
    }
}

impl From<MyOwnColorType> for ANSIColor {
    fn from(value: MyOwnColorType) -> Self {
        ANSIColor::RGB(value.0, value.1, value.2)
    }
}

/// Escape tui-markup syntax in the given string.
pub(crate) fn esc(t: impl Display) -> String {
    let mut out = format!("{t}").replace("\\", "\\\\");
    out = out.replace(">", "\\>");
    out = out.replace("<", "\\<");
    out
}

/// Returns an markup generator for ansi output.
pub fn get_ansi_generator() -> ANSIStringsGenerator<impl Fn(&str) -> Option<ANSIStyle>> {
    ANSIStringsGenerator::new(|tag: &str| match tag {
        "acc" => Some(ANSIStyle::default().fg(ACCENT_COLOR.into())),
        "bgacc" => Some(ANSIStyle::default().on(ACCENT_COLOR.into())),
        "bgred" => Some(ANSIStyle::default().on(RED.into())),
        "red" => Some(ANSIStyle::default().fg(RED.into())),
        "green" => Some(ANSIStyle::default().fg(GREEN.into())),
        "bgyellow" => Some(ANSIStyle::default().on(YELLOW.into())),
        "yellow" => Some(ANSIStyle::default().fg(YELLOW.into())),
        "bggreen" => Some(ANSIStyle::default().fg(BLACK.into()).on(GREEN.into())),
        "command" => Some(
            ANSIStyle::default()
                .fg(COMMAND_FG.into())
                .on(COMMAND_BG.into()),
        ),
        "dim" => Some(ANSIStyle::default().fg(DIM.into())),
        _ => None,
    })
}

/// Returns a markup generator for ratatui widget output.
pub fn get_ratatui_generator() -> RatatuiTextGenerator<impl Fn(&str) -> Option<Style>> {
    RatatuiTextGenerator::new(|tag: &str| match tag {
        "acc" => Some(Style::default().fg(ACCENT_COLOR.into())),
        "bgacc" => Some(Style::default().bg(ACCENT_COLOR.into())),
        "bgyellow" => Some(Style::default().bg(YELLOW.into())),
        "yellow" => Some(Style::default().fg(YELLOW.into())),
        "bgred" => Some(Style::default().bg(RED.into())),
        "red" => Some(Style::default().fg(RED.into())),
        "green" => Some(Style::default().fg(GREEN.into())),
        "bggreen" => Some(Style::default().fg(BLACK.into()).bg(GREEN.into())),
        "command" => Some(Style::default().fg(COMMAND_FG.into()).bg(COMMAND_BG.into())),
        "dim" => Some(Style::default().fg(DIM.into())),
        _ => None,
    })
}
