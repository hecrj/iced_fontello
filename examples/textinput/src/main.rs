mod icon;

use iced::Element;
use iced::widget::{center, row, text, text_input, button};

fn main() -> iced::Result {
    iced::application("Fontello Icons", Example::update, Example::view)
        .font(icon::FONT)
        .run()
}

#[derive(Default)]
struct Example;

impl Example {
    pub fn update(&mut self, _message: ()) {}

    pub fn view(&self) -> Element<()> {
        center(row![
            text_input("Text Input", "").icon(text_input::Icon {
                font: icon::ICON_FONT,
                code_point: icon::Icon::Search.into(),
                size: None,
                spacing: 10.0.into(),
                side: text_input::Side::Left,
            }),
            button(text(char::from(icon::Icon::Save)).font(icon::ICON_FONT))
        ])
        .padding(20)
        .into()
    }
}
