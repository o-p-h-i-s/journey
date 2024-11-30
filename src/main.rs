use iced::{widget::text_editor, Element, Length};

fn main() -> iced::Result {
    iced::application("journey", Editor::update, Editor::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

struct Editor {
    contents: text_editor::Content,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            contents: text_editor::Content::new(),
        }
    }
}

impl Editor {
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.contents.perform(action),
        }
    }

    fn view(&self) -> Element<Message> {
        text_editor(&self.contents)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .into()
    }
}
