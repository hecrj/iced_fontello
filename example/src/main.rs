mod icon;

use iced::Element;
use iced::widget::{center, column, row, text_input};

fn main() -> iced::Result {
    iced::application("Fontello Icons", Example::update, Example::view)
        .font(icon::ICON_FN_FONT)
        .font(icon::ICON_ENUM_FONT)
        .run()
}

#[derive(Default)]
struct Example;

impl Example {
    pub fn update(&mut self, _message: ()) {}

    pub fn view(&self) -> Element<()> {
        center(column![
            row![icon::edit(), icon::save(), icon::trash()].spacing(10),
            row![
                text_input("Text Input", "").icon(icon::text_input_icon(icon::Icon::Search)),
                icon::button_icon(icon::Icon::Erase),
            ]
        ])
        .padding(20)
        .into()
    }
}
