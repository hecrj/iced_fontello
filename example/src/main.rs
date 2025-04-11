mod icon;

use iced::widget::{center, row};
use iced::Element;

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
