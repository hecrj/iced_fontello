// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// c3c0e9102caff0b7aaf530ab362b4cbdd01fa0cddf30328fc8bf24547a78cfc1
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn iced<'a>() -> Text<'a> {
    icon("\u{E800}")
}

pub fn iced_logo<'a>() -> Text<'a> {
    icon("\u{E801}")
}

pub fn rust<'a>() -> Text<'a> {
    icon("\u{E802}")
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
