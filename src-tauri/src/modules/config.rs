use crate::models::AppConfig;
use std::fs;
use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".gemini");
    path.push("antigravity");
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn load_config() -> Result<AppConfig, String> {
    let mut path = get_config_dir();
    path.push("config.json");
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let mut path = get_config_dir();
    path.push("config.json");
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

pub fn load_app_config() -> Result<AppConfig, String> {
    load_config()
}
