// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// 8c26c22502936c82f7e95ac954dfc250e1d042a046bca706486752b9c5bd5898
pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub const ICON_FONT: iced::Font = iced::Font::with_name("example-icons");

pub enum Icon {
    Save,
    Search,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Save => '\u{1F4BE}',
            Icon::Search => '\u{E133}',
        }
    }
}
