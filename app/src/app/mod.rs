mod app;
mod sidebar;
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
    
    sidebar_open: bool,

    explorer: Option<Explorer>,
    settings: Option<Settings>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    LoadApp,
    SetScreen(AppScreen),
    SetSidebarOpen,

    ExplorerMessage(ExplorerMessage),
    SettingsMessage(SettingsMessage),
}
