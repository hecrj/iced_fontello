// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// 1ee4837dfa5e832a5f40be4048333d81b3dcb9723a6549f033480bb4b0f6f2fa
pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

use iced::Font;
use iced::widget::{{Text, text}};

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
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
