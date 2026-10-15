use iced::theme::Palette;
use iced::widget::container;
use iced::{Element, Task, Theme, color};

use super::{App, AppTheme, AppMessage};

const APP_TITLE: &str = "Humboldt";

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        (
            Self {
                theme: AppTheme::Light,
            },

            Task::none(),
        )
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        container(APP_TITLE)
            .width(400.0)
            .height(400.0)
            .into()
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {}
    }

    pub fn title(&self) -> String {
        APP_TITLE.into()
    }

    pub fn theme(&self) -> Theme {
        Theme::custom(
            match self.theme {
                AppTheme::Light => "Light",
                AppTheme::Dark => "Dark",
            },
            match self.theme {
                AppTheme::Light => Palette {
                    background: color!(0xEDF2F8),
                    text: color!(0x27374D),
                    primary: color!(0x526D82),
                    success: color!(0x07C400),
                    warning: color!(0xE77B00),
                    danger: color!(0xF54927),
                },
                AppTheme::Dark => Palette {
                    background: color!(0x282A36),
                    text: color!(0xF8F8F2),
                    primary: color!(0x6272A4),
                    success: color!(0x50FA7B),
                    warning: color!(0xFFB86C),
                    danger: color!(0xFF5555),
                },
            }
        )
    }
}
