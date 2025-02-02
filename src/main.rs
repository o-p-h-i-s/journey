use iced::{widget::horizontal_space, window::Settings, Element, Subscription, Task};

fn main() -> iced::Result {
    iced::application(Journey::title, Journey::update, Journey::view)
        .subscription(Journey::subscription)
        .window(Settings {
            ..Default::default()
        })
        .run_with(Journey::new)
}

struct Journey {}

#[derive(Debug, Clone)]
enum Message {}

impl Journey {
    fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    fn title(&self) -> String {
        "Journey".to_string()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
    }

    fn view(&self) -> Element<Message> {
        horizontal_space().into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
