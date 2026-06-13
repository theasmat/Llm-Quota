use super::registry::*;

use serde::{Deserialize, Serialize};

// Google OAuth configuration


const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const TOKEN_REFRESH_SKEW_SECONDS: i64 = 900;

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub id_token: Option<String>,
    #[serde(skip)]
    pub oauth_client_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
}

impl UserInfo {
    /// Get best display name
    pub fn get_display_name(&self) -> Option<String> {
        // Prefer name
        if let Some(name) = &self.name {
            if !name.trim().is_empty() {
                return Some(name.clone());
            }
        }

        // If name is empty, combine given_name and family_name
        match (&self.given_name, &self.family_name) {
            (Some(given), Some(family)) => Some(format!("{} {}", given, family)),
            (Some(given), None) => Some(given.clone()),
            (None, Some(family)) => Some(family.clone()),
            (None, None) => None,
        }
    }
}


/// Generate OAuth authorization URL with optional client selection.
/// Returns (auth_url, resolved_client_key).
pub fn get_auth_url_with_client(
    redirect_uri: &str,
    state: &str,
    client_key: Option<&str>,
) -> Result<(String, String), String> {
    let client = select_auth_client(client_key)?;

    let scopes = vec![
        "openid",
        "https://www.googleapis.com/auth/cloud-platform",
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/userinfo.profile",
        "https://www.googleapis.com/auth/cclog",
        "https://www.googleapis.com/auth/experimentsandconfigs",
    ]
    .join(" ");

    let params = vec![
        ("client_id", client.client_id.as_str()),
        ("redirect_uri", redirect_uri),
        ("response_type", "code"),
        ("scope", &scopes),
        ("access_type", "offline"),
        ("prompt", "consent"),
        ("include_granted_scopes", "true"),
        ("state", state),
    ];

    let url = url::Url::parse_with_params(AUTH_URL, &params)
        .map_err(|e| format!("Invalid Auth URL: {}", e))?;
    Ok((url.to_string(), client.key))
}

/// Generate OAuth authorization URL using current active client.
pub fn get_auth_url(redirect_uri: &str, state: &str) -> String {
    get_auth_url_with_client(redirect_uri, state, None)
        .map(|(url, _)| url)
        .expect("Failed to build OAuth URL")
}

async fn exchange_code_once(
    code: &str,
    redirect_uri: &str,
    client_cfg: &OAuthClientConfig,
) -> Result<TokenResponse, (Option<reqwest::StatusCode>, String)> {
    // [PHASE 2] ， account_id，
    let client = crate::utils::http::get_long_standard_client();

    let params = [
        ("client_id", client_cfg.client_id.as_str()),
        ("client_secret", client_cfg.client_secret.as_str()),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    tracing::debug!(
        "[OAuth] Sending exchange_code request with User-Agent: {}",
        crate::constants::NATIVE_OAUTH_USER_AGENT
    );

    let response = client
        .post(TOKEN_URL)
        .header(reqwest::header::USER_AGENT, crate::constants::NATIVE_OAUTH_USER_AGENT)
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                (
                    None,
                    format!(
                        "Token exchange request failed: {}. ， Google 。",
                        e
                    ),
                )
            } else {
                (None, format!("Token exchange request failed: {}", e))
            }
        })?;

    if response.status().is_success() {
        let mut token_res = response
            .json::<TokenResponse>()
            .await
            .map_err(|e| (None, format!("Token parsing failed: {}", e)))?;
        token_res.oauth_client_key = Some(client_cfg.key.clone());

        // Add detailed logs
        crate::modules::logger::log_info(&format!(
            "Token exchange successful via [{}]! access_token: {}..., refresh_token: {}",
            client_cfg.key,
            &token_res.access_token.chars().take(20).collect::<String>(),
            if token_res.refresh_token.is_some() {
                "✓"
            } else {
                "✗ Missing"
            }
        ));

        // Log warning if refresh_token is missing
        if token_res.refresh_token.is_none() {
            crate::modules::logger::log_warn(
                "Warning: Google did not return a refresh_token. Potential reasons:\n\
                 1. User has previously authorized this application\n\
                 2. Need to revoke access in Google Cloud Console and retry\n\
                 3. OAuth parameter configuration issue",
            );
        }

        Ok(token_res)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err((
            Some(status),
            format!("Token exchange failed: {}", error_text),
        ))
    }
}

/// Exchange authorization code for token using optional preferred client.
/// When preferred/active client mismatches, fallback to other configured clients.
pub async fn exchange_code_with_client(
    code: &str,
    redirect_uri: &str,
    preferred_client_key: Option<&str>,
) -> Result<TokenResponse, String> {
    let candidates = get_candidate_clients(preferred_client_key);
    if candidates.is_empty() {
        return Err("No OAuth clients configured".to_string());
    }

    let mut attempt_errors: Vec<String> = Vec::new();

    for (idx, client_cfg) in candidates.iter().enumerate() {
        match exchange_code_once(code, redirect_uri, client_cfg).await {
            Ok(token_res) => {
                if idx > 0 {
                    crate::modules::logger::log_info(&format!(
                        "OAuth code exchange recovered via fallback client [{}]",
                        client_cfg.key
                    ));
                }
                return Ok(token_res);
            }
            Err((status_opt, err_msg)) => {
                let should_fallback = status_opt
                    .map(|status| is_client_mismatch_error(status, &err_msg))
                    .unwrap_or(false);

                attempt_errors.push(format!("{} => {}", client_cfg.key, err_msg));

                if should_fallback {
                    crate::modules::logger::log_warn(&format!(
                        "OAuth code exchange failed for client [{}], trying next client: {}",
                        client_cfg.key, err_msg
                    ));
                    continue;
                }

                return Err(format!(
                    "Token exchange failed for client [{}]: {}",
                    client_cfg.key, err_msg
                ));
            }
        }
    }

    Err(format!(
        "Token exchange failed for all OAuth clients: {}",
        attempt_errors.join(" | ")
    ))
}

/// Exchange authorization code for token
pub async fn exchange_code(code: &str, redirect_uri: &str) -> Result<TokenResponse, String> {
    exchange_code_with_client(code, redirect_uri, None).await
}

async fn refresh_access_token_once(
    refresh_token: &str,
    account_id: Option<&str>,
    client_cfg: &OAuthClientConfig,
) -> Result<TokenResponse, (Option<reqwest::StatusCode>, String)> {
    // [PHASE 2]  account_id 
    let client = crate::utils::http::get_long_standard_client();

    let params = [
        ("client_id", client_cfg.client_id.as_str()),
        ("client_secret", client_cfg.client_secret.as_str()),
        ("refresh_token", refresh_token),
        ("grant_type", "refresh_token"),
    ];

    // [FIX #1583] ， Docker 
    if let Some(id) = account_id {
        crate::modules::logger::log_info(&format!("Refreshing Token for account: {}...", id));
    } else {
        crate::modules::logger::log_info("Refreshing Token for generic request (no account_id)...");
    }

    tracing::debug!(
        "[OAuth] Sending refresh_access_token request with User-Agent: {}",
        crate::constants::NATIVE_OAUTH_USER_AGENT
    );

    let response = client
        .post(TOKEN_URL)
        .header(
            reqwest::header::USER_AGENT,
            crate::constants::NATIVE_OAUTH_USER_AGENT,
        )
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                (
                    None,
                    format!(
                        "Refresh request failed: {}.  Google ，。",
                        e
                    ),
                )
            } else {
                (None, format!("Refresh request failed: {}", e))
            }
        })?;

    if response.status().is_success() {
        let mut token_data = response
            .json::<TokenResponse>()
            .await
            .map_err(|e| (None, format!("Refresh data parsing failed: {}", e)))?;
        token_data.oauth_client_key = Some(client_cfg.key.clone());

        crate::modules::logger::log_info(&format!(
            "Token refreshed successfully via [{}]! Expires in: {} seconds",
            client_cfg.key, token_data.expires_in
        ));
        Ok(token_data)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        
        #[cfg(debug_assertions)]
        {
            crate::modules::logger::log_error(&format!("OAuth Refresh Error [{}]: {}", status, error_text));
        }

        Err((Some(status), format!("Refresh failed: {}", error_text)))
    }
}

/// Refresh access_token using refresh_token with optional preferred OAuth client key.
/// If client mismatch occurs, it retries with other configured clients.
pub async fn refresh_access_token_with_client(
    refresh_token: &str,
    account_id: Option<&str>,
    preferred_client_key: Option<&str>,
) -> Result<TokenResponse, String> {
    let candidates = get_candidate_clients(preferred_client_key);
    if candidates.is_empty() {
        return Err("No OAuth clients configured".to_string());
    }

    let mut attempt_errors: Vec<String> = Vec::new();

    for (idx, client_cfg) in candidates.iter().enumerate() {
        match refresh_access_token_once(refresh_token, account_id, client_cfg).await {
            Ok(token_res) => {
                if idx > 0 {
                    crate::modules::logger::log_info(&format!(
                        "Refresh recovered via fallback OAuth client [{}]",
                        client_cfg.key
                    ));
                }
                return Ok(token_res);
            }
            Err((status_opt, err_msg)) => {
                let should_fallback = status_opt
                    .map(|status| is_client_mismatch_error(status, &err_msg))
                    .unwrap_or(false);

                attempt_errors.push(format!("{} => {}", client_cfg.key, err_msg));

                if should_fallback {
                    crate::modules::logger::log_warn(&format!(
                        "Refresh failed for client [{}], trying next client: {}",
                        client_cfg.key, err_msg
                    ));
                    continue;
                }

                return Err(format!(
                    "Refresh failed for client [{}]: {}",
                    client_cfg.key, err_msg
                ));
            }
        }
    }

    Err(format!(
        "Refresh failed for all OAuth clients: {}",
        attempt_errors.join(" | ")
    ))
}

/// Refresh access_token using refresh_token
pub async fn refresh_access_token(
    refresh_token: &str,
    account_id: Option<&str>,
) -> Result<TokenResponse, String> {
    refresh_access_token_with_client(refresh_token, account_id, None).await
}

/// Get user info
pub async fn get_user_info(
    access_token: &str,
    _account_id: Option<&str>,
) -> Result<UserInfo, String> {
    let client = crate::utils::http::get_client();

    let response = client
        .get(USERINFO_URL)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("User info request failed: {}", e))?;

    if response.status().is_success() {
        response
            .json::<UserInfo>()
            .await
            .map_err(|e| format!("User info parsing failed: {}", e))
    } else {
        let error_text = response.text().await.unwrap_or_default();
        Err(format!("Failed to get user info: {}", error_text))
    }
}

/// Check and refresh Token if needed
/// Returns the latest access_token
pub async fn ensure_fresh_token(
    current_token: &crate::models::TokenData,
    account_id: Option<&str>,
) -> Result<crate::models::TokenData, String> {
    let now = chrono::Local::now().timestamp();

    // Keep enough validity to avoid immediate post-switch refresh failure.
    if current_token.expiry_timestamp > now + TOKEN_REFRESH_SKEW_SECONDS {
        return Ok(current_token.clone());
    }

    // Need to refresh
    crate::modules::logger::log_info(&format!(
        "Token expiring soon for account {:?}, refreshing...",
        account_id
    ));
    let response = refresh_access_token_with_client(
        &current_token.refresh_token,
        account_id,
        current_token.oauth_client_key.as_deref(),
    )
    .await?;

    let oauth_client_key =
        normalize_refreshed_oauth_client_key(current_token, response.oauth_client_key.clone());

    // Construct new TokenData
    Ok(crate::models::TokenData::new(
        response.access_token,
        current_token.refresh_token.clone(), // refresh_token may not be returned on refresh
        response.expires_in,
        current_token.email.clone(),
        current_token.project_id.clone(), // Keep original project_id
        None,                             // session_id will be generated in token_manager
        current_token.is_gcp_tos,
        response.id_token.or(current_token.id_token.clone()), // Use new id_token or keep old one
    )
    .with_oauth_client_key(oauth_client_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_auth_url_contains_state() {
        let redirect_uri = "http://localhost:8080/callback";
        let state = "test-state-123456";
        let url = get_auth_url(redirect_uri, state);

        assert!(url.contains("state=test-state-123456"));
        assert!(url.contains("redirect_uri=http%3A%2F%2Flocalhost%3A8080%2Fcallback"));
        assert!(url.contains("response_type=code"));
    }
}
