use std::env;

use iced::{
    widget::{self, button, container, pick_list, text},
    Length, Sandbox, Settings,
};
use iced_aw::{floating_element::Anchor, FloatingElement};

fn main() -> iced::Result {
    Floating::run(Settings::default())
}
pub static CHOICES: &[&str] = &["1", "2", "3", "4"];

struct Floating {
    include_float: bool,
}

impl Sandbox for Floating {
    type Message = ();

    fn new() -> Self {
        let args = env::args();
        Floating {
            include_float: args.len() > 1,
        }
    }

    fn title(&self) -> String {
        String::from("Test for floating element")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let list_container = container(widget::column![pick_list(CHOICES, Some("1"), |_| ())])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        if self.include_float {
            FloatingElement::new(list_container, || {
                button(text("X").size(24)).padding(6).into()
            })
            .anchor(Anchor::NorthWest)
            .into()
        } else {
            list_container.into()
        }
    }
}
