use crate::buffer;

use iced::widget::{container, pane_grid, PaneGrid};
use iced::{Element, Length};

#[derive(Debug)]
pub struct Pane {
    state: pane_grid::State<buffer::Buffer>,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Buffer(buffer::Message),
    Clicked(pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    CloseFocused,
}

impl Pane {
    pub fn new() -> Self {
        let (state, pane) = pane_grid::State::new(buffer::Buffer::new());

        Self {
            state,
            focus: Some(pane),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Buffer(message) => {
                if let Some(pane) = self.focus {
                    if let Some(buffer) = self.state.get_mut(pane) {
                        buffer.update(message);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let res = self.state.split(axis, pane, buffer::Buffer::new());

                    if let Some((pane, _)) = res {
                        self.focus = Some(pane);
                    }
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some((_, sibling)) = self.state.close(pane) {
                        self.focus = Some(sibling);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.state, |_pane, buffer, _is_maximized| {
            pane_grid::Content::new(buffer.view().map(Message::Buffer))
        })
        .on_click(Message::Clicked)
        .width(Length::Fill)
        .height(Length::Fill);

        container(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
