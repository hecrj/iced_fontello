// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// 98d2fe4433f22b49d44ab0e3f801adada1dfd901f77d56df8ce3cb78525fb37f
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn iced<'a>() -> Text<'a> {
    icon("\u{E800}")
}

pub fn rust<'a>() -> Text<'a> {
    icon("\u{E801}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E10A}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("example-icons"))
}
