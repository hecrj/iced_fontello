mod icon_fn;
pub use icon_fn::FONT as ICON_FN_FONT;
pub use icon_fn::*;

mod icon_enum;
pub use icon_enum::FONT as ICON_ENUM_FONT;
pub use icon_enum::Icon;

use iced::{
    Font,
    widget::{Button, Text, text_input},
};
use icon_enum::ICON_FONT;

pub fn text_icon<'a>(icon: Icon) -> Text<'a> {
    Text::new(char::from(icon)).font(ICON_FONT)
}

pub fn button_icon<'a, Message>(icon: Icon) -> Button<'a, Message> {
    Button::new(text_icon(icon))
}

pub fn text_input_icon(icon: Icon) -> text_input::Icon<Font> {
    text_input::Icon {
        font: ICON_FONT,
        code_point: icon.into(),
        size: None,
        spacing: 0.0,
        side: text_input::Side::Left,
    }
}
