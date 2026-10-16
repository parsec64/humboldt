use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use crate::data::settings::SettingsInfo;


// App
pub const APP_NAME: &str = "Humboldt";
pub const APP_VERSION: &str = "0.1.0";

// Data
pub const DATA_DIR_NAME: &str = ".humboldt";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

// Intialized once
pub static DATA_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();

// Cached settings
pub static SETTINGS: OnceLock<Mutex<SettingsInfo>> = OnceLock::new();
