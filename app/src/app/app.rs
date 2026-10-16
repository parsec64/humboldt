use iced::theme::Palette;
use iced::widget::{row, container, text};
use iced::{Border, Element, Length, Task, Theme, color};

use super::{App, AppMessage};
use super::explorer::Explorer;
use super::settings::Settings;

use crate::data::settings::{AppScreen, AppTheme};
use crate::components::loading_screen::loading_screen_view;
use crate::constants::APP_NAME;


impl App<'static> {
    pub fn new() -> (Self, Task<AppMessage>) {
        (
            Self {
                title: APP_NAME,
                theme: AppTheme::Dark,
                screen: AppScreen::Explorer,

                explorer: None,
                settings: None,
            },

            Task::none(),
        )
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        row![
            // Sidebar
            self.sidebar_view(),

            // Screens
            match self.screen {
                AppScreen::Explorer => match &self.explorer {
                    Some(dashboard) => dashboard.view().map(|m| AppMessage::ExplorerMessage(m)),
                    None => loading_screen_view(),
                },
                AppScreen::Settings => match &self.settings {
                    Some(settings) => settings.view().map(|m| AppMessage::SettingsMessage(m)),
                    None => loading_screen_view(),
                },
            }
        ]
        .into()
    }

    pub fn sidebar_view(&self) -> Element<'_, AppMessage> {
        container(
            text("Side bar")
        )
        .height(Length::Fill)
        .width(250)
        .padding(10)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::LoadApp => {
                let (explorer_view, explorer_task) = Explorer::new();
                let (settings_view, settings_task) = Settings::new();

                self.explorer = Some(explorer_view);
                self.settings = Some(settings_view);

                Task::batch([
                    explorer_task.map(|m| AppMessage::ExplorerMessage(m)),
                    settings_task.map(|m| AppMessage::SettingsMessage(m)),
                ])
            },
            AppMessage::ExplorerMessage(dashboard_m) => match &mut self.explorer {
                Some(dashboard) => dashboard.update(dashboard_m).map(|m| AppMessage::ExplorerMessage(m)),
                None => Task::none(),
            },
            AppMessage::SettingsMessage(settings_m) => match &mut self.settings {
                Some(settings) => settings.update(settings_m).map(|m| AppMessage::SettingsMessage(m)),
                None => Task::none(),
            },
        }
    }

    pub fn title(&self) -> String {
        match self.screen {
            AppScreen::Explorer => format!("{} - Explorer", self.title),
            AppScreen::Settings => format!("{} - Settings", self.title)
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::custom(
            match self.theme {
                AppTheme::Light => "Shelf Light",
                AppTheme::Dark => "Shelf Dark",
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
