use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Polish,
    Russian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub language: Language,
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: f32,
}

fn default_refresh_rate() -> f32 {
    2.5
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            language: Language::English,
            refresh_rate: 2.5,
        }
    }
}

fn config_path() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let dir = PathBuf::from(appdata).join("TaskExplorer");
        let _ = fs::create_dir_all(&dir);
        dir.join("config.json")
    } else {
        PathBuf::from("config.json")
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let path = config_path();
        if let Ok(data) = fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    pub fn save(&self) {
        let path = config_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}
