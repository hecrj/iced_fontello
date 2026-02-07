mod icon;

use iced::Element;
use iced::widget::{center, row};

fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(icon::FONT)
        .run()
}

#[derive(Default)]
struct Example;

impl Example {
    pub fn update(&mut self, _message: ()) {}

    pub fn view(&self) -> Element<'_, ()> {
        center(row![icon::edit(), icon::save(), icon::trash()].spacing(10))
            .padding(20)
            .into()
    }
}
