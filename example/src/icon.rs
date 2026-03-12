// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// d24460a00249b2acd0ccc64c3176452c546ad12d1038974e974d7bdb4cdb4a8f
use iced::Font;
use iced::widget::{Text, text};
use iced::widget::text::Catalog;

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a, Theme: Catalog + 'a>() -> Text<'a, Theme> {
    icon("\u{270E}")
}

pub fn save<'a, Theme: Catalog + 'a>() -> Text<'a, Theme> {
    icon("\u{1F4BE}")
}

pub fn trash<'a, Theme: Catalog + 'a>() -> Text<'a, Theme> {
    icon("\u{E10A}")
}

fn icon<'a, Theme: Catalog + 'a>(codepoint: &'a str) -> Text<'a, Theme> {
    text(codepoint).font(Font::with_name("example-icons"))
}
