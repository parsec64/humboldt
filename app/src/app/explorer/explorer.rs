use iced::widget::{container, text};
use iced::{Background, Element, Length, Task, color};
use iced::widget::stack;

use super::{Explorer, ExplorerMessage};


impl Explorer {
    pub fn new() -> (Self, Task<ExplorerMessage>) {
        (
            Self {},

            Task::done(ExplorerMessage::LoadExplorer),
        )
    }

    pub fn view(&self) -> Element<'_, ExplorerMessage> {
        stack![
            container(
                text("Explorer")
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(color!(0x7b8b9b))),
                ..Default::default()
            })
        ]
        .into()
    }

    pub fn update(&mut self, message: ExplorerMessage) -> Task<ExplorerMessage> {
        match message {
            ExplorerMessage::LoadExplorer => {
                Task::none()
            },
        }
    }
}
