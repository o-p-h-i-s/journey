mod buffer;
mod pane;

use iced::window::Settings;
use iced::{keyboard, Element, Subscription, Task};

fn main() -> iced::Result {
    iced::application(Journey::title, Journey::update, Journey::view)
        .subscription(Journey::subscription)
        .window(Settings {
            ..Default::default()
        })
        .run_with(Journey::new)
}

#[derive(Debug)]
struct Journey {
    pane: pane::Pane,
}

#[derive(Debug, Clone)]
enum Message {
    Pane(pane::Message),
}

impl Journey {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                pane: pane::Pane::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        "Journey".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Pane(message) => {
                self.pane.update(message);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.pane.view().map(Message::Pane)
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| keybinding(key, modifiers))
    }
}

fn keybinding(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    use iced::widget::pane_grid::Axis;
    use keyboard::Key;

    if !modifiers.control() {
        return None;
    }

    match key.as_ref() {
        Key::Character("v") => Some(Message::Pane(pane::Message::SplitFocused(Axis::Vertical))),
        Key::Character("h") => Some(Message::Pane(pane::Message::SplitFocused(Axis::Horizontal))),
        Key::Character("c") => Some(Message::Pane(pane::Message::CloseFocused)),
        _ => None,
    }
}
