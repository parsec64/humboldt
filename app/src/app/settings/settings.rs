use iced::widget::{container, stack, text};
use iced::{Background, Element, Length, Task, color};

use super::{Settings, SettingsMessage};


impl Settings {
    pub fn new() -> (Self, Task<SettingsMessage>) {
        (
            Self {},

            Task::done(SettingsMessage::LoadSettings),
        )
    }

    pub fn view(&self) -> Element<'_, SettingsMessage> {
        stack![
            container(
                text("Explorer")
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(color!(0x1b8b1b))),
                ..Default::default()
            })
        ]
        .into()
    }

    pub fn update(&mut self, message: SettingsMessage) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::LoadSettings => {
                Task::none()
            },
        }
    }
}
