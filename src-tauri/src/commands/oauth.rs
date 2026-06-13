use crate::models::{Account, TokenData};
use crate::modules;
use crate::modules::oauth::OAuthClientDescriptor;

#[tauri::command]
pub async fn start_oauth_login(app: tauri::AppHandle) -> Result<Account, String> {
    // Note: since we haven't modularized oauth_server yet, we use modules::oauth_server
    let token_res = modules::oauth::start_oauth_flow(Some(app), None).await?;

    let user_info = modules::oauth::get_user_info(&token_res.access_token, None).await?;

    let token = TokenData::new(
        token_res.access_token.clone(),
        token_res.refresh_token.unwrap_or_default(),
        token_res.expires_in,
        Some(user_info.email.clone()),
        None,
        None,
        false,
        token_res.id_token.clone(),
    )
    .with_oauth_client_key(token_res.oauth_client_key.clone());

    let mut account = modules::account::upsert_account(
        user_info.email.clone(),
        user_info.get_display_name(),
        token,
    )?;

    if let Ok((quota_data, new_project_id)) =
        modules::quota::fetch_quota(&token_res.access_token, &account.email, Some(&account.id))
            .await
    {
        account.quota = Some(quota_data);
        if let Some(pid) = new_project_id {
            account.token.project_id = Some(pid);
        }
        let _ = modules::account::save_account(&account);
    }

    Ok(account)
}

#[tauri::command]
pub async fn prepare_oauth_url(app: tauri::AppHandle) -> Result<String, String> {
    modules::oauth::prepare_oauth_url(Some(app), None).await
}

#[tauri::command]
pub async fn complete_oauth_login() -> Result<Account, String> {
    let token_res = modules::oauth::complete_oauth_flow(None).await?;

    let user_info = modules::oauth::get_user_info(&token_res.access_token, None).await?;

    let token = TokenData::new(
        token_res.access_token.clone(),
        token_res.refresh_token.unwrap_or_default(),
        token_res.expires_in,
        Some(user_info.email.clone()),
        None,
        None,
        false,
        token_res.id_token.clone(),
    )
    .with_oauth_client_key(token_res.oauth_client_key.clone());

    let mut account = modules::account::upsert_account(
        user_info.email.clone(),
        user_info.get_display_name(),
        token,
    )?;

    if let Ok((quota_data, new_project_id)) =
        modules::quota::fetch_quota(&token_res.access_token, &account.email, Some(&account.id))
            .await
    {
        account.quota = Some(quota_data);
        if let Some(pid) = new_project_id {
            account.token.project_id = Some(pid);
        }
        let _ = modules::account::save_account(&account);
    }

    Ok(account)
}

#[tauri::command]
pub fn cancel_oauth_login() {
    modules::oauth::cancel_oauth_flow();
}

#[tauri::command]
pub async fn submit_oauth_code(code: String, state: Option<String>) -> Result<(), String> {
    modules::oauth::submit_oauth_code(code, state).await
}

#[tauri::command]
pub fn list_oauth_clients() -> Result<Vec<OAuthClientDescriptor>, String> {
    modules::oauth::list_oauth_clients()
}

#[tauri::command]
pub fn get_active_oauth_client() -> Result<String, String> {
    modules::oauth::get_active_oauth_client_key()
}

#[tauri::command]
pub fn set_active_oauth_client(client_key: String) -> Result<(), String> {
    modules::oauth::set_active_oauth_client_key(&client_key)
}
