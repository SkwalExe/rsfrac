use ansi_term::{Color as ANSIColor, Style as ANSIStyle};
use ratatui::style::{Color, Style};
use tui_markup::generator::{ANSIStringsGenerator, RatatuiTextGenerator};

const ACCENT_COLOR: MyOwnColorType = MyOwnColorType(162, 119, 255);
const RED: MyOwnColorType = MyOwnColorType(255, 109, 109);
const GREEN: MyOwnColorType = MyOwnColorType(88, 224, 178);
const DIM: MyOwnColorType = MyOwnColorType(42, 42, 42);
const COMMAND_BG: MyOwnColorType = MyOwnColorType(34, 40, 39);
const COMMAND_FG: MyOwnColorType = MyOwnColorType(217, 214, 207);

struct MyOwnColorType(u8, u8, u8);

impl Into<Color> for MyOwnColorType {
    fn into(self) -> Color {
        Color::Rgb(self.0, self.1, self.2)
    }
}

impl Into<ANSIColor> for MyOwnColorType {
    fn into(self) -> ANSIColor {
        ANSIColor::RGB(self.0, self.1, self.2)
    }
}

pub fn get_ansi_generator() -> ANSIStringsGenerator<impl Fn(&str) -> Option<ANSIStyle>> {
    ANSIStringsGenerator::new(|tag: &str| match tag {
        "acc" => Some(ANSIStyle::default().fg(ACCENT_COLOR.into())),
        "bgacc" => Some(ANSIStyle::default().on(ACCENT_COLOR.into())),
        "bgred" => Some(ANSIStyle::default().on(RED.into())),
        "red" => Some(ANSIStyle::default().fg(RED.into())),
        "green" => Some(ANSIStyle::default().fg(GREEN.into())),
        "bggreen" => Some(ANSIStyle::default().on(GREEN.into())),
        "command" => Some(
            ANSIStyle::default()
                .fg(COMMAND_FG.into())
                .on(COMMAND_BG.into()),
        ),
        "dim" => Some(ANSIStyle::default().fg(DIM.into())),
        _ => None,
    })
}

pub fn get_ratatui_generator() -> RatatuiTextGenerator<impl Fn(&str) -> Option<Style>> {
    RatatuiTextGenerator::new(|tag: &str| match tag {
        "acc" => Some(Style::default().fg(ACCENT_COLOR.into())),
        "bgacc" => Some(Style::default().bg(ACCENT_COLOR.into())),
        "bgred" => Some(Style::default().bg(RED.into())),
        "red" => Some(Style::default().fg(RED.into())),
        "green" => Some(Style::default().fg(GREEN.into())),
        "bggreen" => Some(Style::default().bg(GREEN.into())),
        "command" => Some(Style::default().fg(COMMAND_FG.into()).bg(COMMAND_BG.into())),
        "dim" => Some(Style::default().fg(DIM.into())),
        _ => None,
    })
}
