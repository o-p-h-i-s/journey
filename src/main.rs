use std::path::PathBuf;

use iced::{
    widget::{button, column, container, horizontal_space, row, text_editor},
    Element, Length,
};

fn main() -> iced::Result {
    iced::application("journey", Editor::update, Editor::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Open,
}

struct Editor {
    path: Option<PathBuf>,
    contents: text_editor::Content,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            path: None,
            contents: text_editor::Content::new(),
        }
    }
}

impl Editor {
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.contents.perform(action),
            Message::Open => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let new = button("New");
        let open = button("Open").on_press(Message::Open);
        let save = button("Save");

        let controls = row![horizontal_space(), new, open, save].spacing(5);

        let editor = text_editor(&self.contents)
            .on_action(Message::Edit)
            .height(Length::Fill);

        let main = column![controls, editor].spacing(5);

        container(main).padding(5).into()
    }
}
