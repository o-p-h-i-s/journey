use iced::{Element, Task};
use screen::{editor, Screen};

mod screen;

fn main() -> iced::Result {
    iced::application("Journey", Journey::update, Journey::view).run_with(Journey::new)
}

#[derive(Debug, Clone)]
enum Message {
    Editor(editor::Message),
}

struct Journey {
    screen: screen::Screen,
}

impl Journey {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Editor(editor::Editor::new()),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Editor(message) => {
                let Screen::Editor(editor) = &mut self.screen;
                editor.update(message);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Editor(editor) => editor.view().map(Message::Editor),
        }
    }
}
