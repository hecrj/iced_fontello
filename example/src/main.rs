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
        center(
            row![
                "edit:",
                icon::edit().size(30),
                ", save:",
                icon::save().size(30),
                ", trash:",
                icon::trash().size(30),
                ", iced:",
                icon::iced().size(30),
                ", rust:",
                icon::rust().size(30),
                ", end!",
            ]
            .align_y(iced::Center)
            .spacing(10),
        )
        .padding(20)
        .into()
    }
}
