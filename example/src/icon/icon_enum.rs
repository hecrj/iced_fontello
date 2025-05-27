// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../../fonts/enum-icons.toml
// c7b9306b41b99f8e906c7136d1cc00d67cb3fc087ebe33c95f1002684f4c5b6e
pub const FONT: &[u8] = include_bytes!("../../fonts/enum-icons.ttf");

pub const ICON_FONT: iced::Font = iced::Font::with_name("enum-icons");

pub enum Icon {
    Erase,
    Search,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Erase => '\u{232B}',
            Icon::Search => '\u{E133}',
        }
    }
}
