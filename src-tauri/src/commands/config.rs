use crate::models::AppConfig;
use crate::modules;

#[tauri::command]
pub async fn load_config() -> Result<AppConfig, String> {
    modules::config::load_config()
}

#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    modules::config::save_config(&config)
}
