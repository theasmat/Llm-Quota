use crate::modules::integration;

#[tauri::command]
pub async fn check_local_integrations(email: String) -> Result<Vec<String>, String> {
    Ok(integration::check_local_integrations(&email))
}
