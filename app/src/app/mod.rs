mod app;
mod explorer;
mod settings;

use super::data::settings::{AppScreen, AppTheme};
use explorer::{Explorer, ExplorerMessage};
use settings::{Settings, SettingsMessage};


#[derive(Debug)]
pub struct App<'a: 'static> {
    title: &'a str,
    theme: AppTheme,
    screen: AppScreen,

    explorer: Option<Explorer>,
    settings: Option<Settings>,
}

#[derive(Debug)]
pub enum AppMessage {
    LoadApp,

    ExplorerMessage(ExplorerMessage),
    SettingsMessage(SettingsMessage),
}
