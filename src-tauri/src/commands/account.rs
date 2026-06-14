use crate::models::{Account, QuotaData, TokenData};
use crate::modules;

#[tauri::command]
pub async fn list_accounts() -> Result<Vec<Account>, String> {
    modules::account::list_accounts()
}

#[tauri::command]
pub async fn add_account(
    _app: tauri::AppHandle,
    _email: String,
    refresh_token: String,
) -> Result<Account, String> {
    let temp_account_id = uuid::Uuid::new_v4().to_string();
    let token_res =
        modules::oauth::refresh_access_token(&refresh_token, Some(&temp_account_id)).await?;
    let user_info =
        modules::oauth::get_user_info(&token_res.access_token, Some(&temp_account_id)).await?;

    let token = TokenData::new(
        token_res.access_token.clone(),
        refresh_token.to_string(),
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
pub async fn delete_account(_app: tauri::AppHandle, account_id: String) -> Result<(), String> {
    modules::account::delete_account(&account_id)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_accounts(
    _app: tauri::AppHandle,
    account_ids: Vec<String>,
) -> Result<(), String> {
    modules::account::delete_accounts(&account_ids)?;
    Ok(())
}

#[tauri::command]
pub async fn reorder_accounts(account_ids: Vec<String>) -> Result<(), String> {
    modules::account::reorder_accounts(&account_ids)?;
    Ok(())
}

#[tauri::command]
pub async fn switch_account(
    _app: tauri::AppHandle,
    _account_id: String,
    _target_ide: Option<String>,
) -> Result<(), String> {
    // legacy switch account
    Ok(())
}

#[tauri::command]
pub async fn export_accounts() -> Result<String, String> {
    let accounts = modules::account::list_accounts()?;
    let json = serde_json::to_string_pretty(&accounts).map_err(|e| e.to_string())?;
    Ok(json)
}

#[tauri::command]
pub async fn get_current_account() -> Result<Option<Account>, String> {
    let accounts = modules::account::list_accounts()?;
    Ok(accounts.into_iter().next()) // simple fallback
}

#[tauri::command]
pub async fn fetch_account_quota(
    _app: tauri::AppHandle,
    account_id: String,
) -> Result<QuotaData, String> {
    let mut account = modules::account::get_account(&account_id)
        .ok_or_else(|| "Account not found".to_string())?;

    // Smart token refresh: only if expired or expiring soon
    if !account.token.refresh_token.is_empty() {
        if let Ok(fresh_token) =
            modules::oauth::ensure_fresh_token(&account.token, Some(&account.id)).await
        {
            account.token = fresh_token;
            let _ = modules::account::save_account(&account);
        }
    }

    let (quota_data, pid) = modules::quota::fetch_quota(
        &account.token.access_token,
        &account.email,
        Some(&account.id),
    )
    .await
    .map_err(|e| e.to_string())?;

    // If 403, force-refresh the token and retry once — the 403 often means the token
    // was silently invalidated by Google, not that the account is truly banned.
    if quota_data.is_forbidden && !account.token.refresh_token.is_empty() {
        crate::modules::logger::log_info(&format!(
            "Got 403 for {}, force-refreshing token and retrying...",
            account.email
        ));
        match modules::oauth::refresh_access_token(&account.token.refresh_token, Some(&account.id))
            .await
        {
            Ok(token_res) => {
                account.token.access_token = token_res.access_token.clone();
                account.token.expires_in = token_res.expires_in;
                account.token.expiry_timestamp =
                    chrono::Utc::now().timestamp() + token_res.expires_in;
                if let Some(key) = token_res.oauth_client_key.clone() {
                    account.token.oauth_client_key = Some(key);
                }
                let _ = modules::account::save_account(&account);

                // Retry quota fetch with the fresh token
                match modules::quota::fetch_quota(
                    &account.token.access_token,
                    &account.email,
                    Some(&account.id),
                )
                .await
                {
                    Ok((retry_quota, retry_pid)) => {
                        account.quota = Some(retry_quota.clone());
                        if let Some(p) = retry_pid {
                            account.token.project_id = Some(p);
                        }
                        let _ = modules::account::save_account(&account);
                        return Ok(retry_quota);
                    }
                    Err(e) => {
                        crate::modules::logger::log_warn(&format!(
                            "Retry after token refresh still failed for {}: {}",
                            account.email, e
                        ));
                    }
                }
            }
            Err(e) => {
                crate::modules::logger::log_warn(&format!(
                    "Force token refresh failed for {}: {}",
                    account.email, e
                ));
            }
        }
    }

    // Save whatever we got (might still be forbidden if retry failed)
    account.quota = Some(quota_data.clone());
    if let Some(p) = pid {
        account.token.project_id = Some(p);
    }
    let _ = modules::account::save_account(&account);
    Ok(quota_data)
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RefreshStats {
    pub success: u32,
    pub failed: u32,
    pub details: Vec<String>,
}

#[tauri::command]
pub async fn refresh_all_quotas(
    _app: tauri::AppHandle,
) -> Result<RefreshStats, String> {
    let mut stats = RefreshStats { success: 0, failed: 0, details: Vec::new() };
    let accounts = modules::account::list_accounts()?;
    for mut acc in accounts {
        // Smart token refresh: only if expired or expiring soon
        if !acc.token.refresh_token.is_empty() {
            if let Ok(fresh_token) =
                modules::oauth::ensure_fresh_token(&acc.token, Some(&acc.id)).await
            {
                acc.token = fresh_token;
                let _ = modules::account::save_account(&acc);
            }
        }
        match modules::quota::fetch_quota(&acc.token.access_token, &acc.email, Some(&acc.id)).await
        {
            Ok((q, pid)) => {
                // If 403, force-refresh token and retry once
                if q.is_forbidden && !acc.token.refresh_token.is_empty() {
                    if let Ok(token_res) = modules::oauth::refresh_access_token(
                        &acc.token.refresh_token,
                        Some(&acc.id),
                    )
                    .await
                    {
                        acc.token.access_token = token_res.access_token.clone();
                        acc.token.expires_in = token_res.expires_in;
                        acc.token.expiry_timestamp =
                            chrono::Utc::now().timestamp() + token_res.expires_in;
                        if let Some(key) = token_res.oauth_client_key.clone() {
                            acc.token.oauth_client_key = Some(key);
                        }
                        let _ = modules::account::save_account(&acc);

                        if let Ok((retry_q, retry_pid)) = modules::quota::fetch_quota(
                            &acc.token.access_token,
                            &acc.email,
                            Some(&acc.id),
                        )
                        .await
                        {
                            acc.quota = Some(retry_q.clone());
                            if let Some(p) = retry_pid {
                                acc.token.project_id = Some(p);
                            }
                            let _ = modules::account::save_account(&acc);
                            stats.success += 1;
                            continue;
                        }
                    }
                }
                acc.quota = Some(q.clone());
                if let Some(p) = pid {
                    acc.token.project_id = Some(p);
                }
                let _ = modules::account::save_account(&acc);
                stats.success += 1;
            }
            Err(e) => {
                stats.failed += 1;
                stats.details.push(format!("{}: {}", acc.email, e));
            }
        }
    }
    Ok(stats)
}

#[tauri::command]
pub async fn import_from_db() -> Result<Vec<Account>, String> {
    // Import accounts from our own DB file (no-op: just return existing accounts)
    modules::account::list_accounts()
}

#[tauri::command]
pub async fn import_v1_accounts() -> Result<Vec<Account>, String> {
    // Legacy v1 import: try to read from old path
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".gemini");
    path.push("accounts.json");
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_custom_db(path: String) -> Result<Vec<Account>, String> {
    if !std::path::Path::new(&path).exists() {
        return Err("File not found".to_string());
    }
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}
