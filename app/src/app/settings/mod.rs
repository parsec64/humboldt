mod settings;


#[derive(Debug)]
pub struct Settings {

}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    LoadSettings,
}
