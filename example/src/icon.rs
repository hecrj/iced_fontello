// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// e6b6b05a99285736e22dd3b5fe11de6d5158add84b89e3b58a650d69cbd8cb7b
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
