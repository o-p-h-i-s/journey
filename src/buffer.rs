use iced::{
    widget::{container, horizontal_space, row, text},
    Element, Length,
};

#[derive(Debug)]
pub struct Buffer {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Buffer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view(&self) -> Element<Message> {
        container(row![
            horizontal_space(),
            text!("buffer"),
            horizontal_space()
        ])
        .center(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
