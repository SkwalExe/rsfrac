use ratatui::style::{Color, Style};
use ansi_term::{Style as ANSIStyle, Color as ANSIColor};
use tui_markup::generator::{ANSIStringsGenerator, RatatuiTextGenerator};

const ANSI_ACCENT_COLOR: ANSIColor = ANSIColor::RGB(162, 119, 255);
const ANSI_RED: ANSIColor = ANSIColor::RGB(255, 109, 109);
const ANSI_DIM: ANSIColor = ANSIColor::RGB(42, 42, 42);
const ACCENT_COLOR: Color = Color::Rgb(162, 119, 255);
const RED: Color = Color::Rgb(255, 109, 109);
const DIM: Color = Color::Rgb(255, 109, 109);

pub fn get_ansi_generator() -> ANSIStringsGenerator<impl Fn(&str) -> Option<ANSIStyle>> {
    ANSIStringsGenerator::new(|tag: &str| match tag {
        "acc" => Some(ANSIStyle::default().fg(ANSI_ACCENT_COLOR)),
        "bgacc" => Some(ANSIStyle::default().on(ANSI_ACCENT_COLOR)),
        "bgred" => Some(ANSIStyle::default().on(ANSI_RED)),
        "red" => Some(ANSIStyle::default().fg(ANSI_RED)),
        "dim" => Some(ANSIStyle::default().fg(ANSI_DIM)),
        _ => None,
    })
}

pub fn get_ratatui_generator() -> RatatuiTextGenerator<impl Fn(&str) -> Option<Style>> {
    RatatuiTextGenerator::new(|tag: &str| match tag {
        "acc" => Some(Style::default().fg(ACCENT_COLOR)),
        "bgacc" => Some(Style::default().bg(ACCENT_COLOR)),
        "bgred" => Some(Style::default().bg(RED)),
        "red" => Some(Style::default().fg(RED)),
        "dim" => Some(Style::default().fg(DIM)),
        _ => None,
    })
}
