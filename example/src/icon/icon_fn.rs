// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../../fonts/fn-icons.toml
// 0e6ffba66f7e4083d93af9c9e72ad1d3743fe8c9a34701d57d17355811b3d13a
pub const FONT: &[u8] = include_bytes!("../../fonts/fn-icons.ttf");

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
    text(codepoint).font(Font::with_name("fn-icons"))
}
