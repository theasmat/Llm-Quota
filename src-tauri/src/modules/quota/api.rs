use super::types::*;
use crate::models::QuotaData;
use reqwest;

use serde_json::json;

// Quota API endpoints (fallback order: Sandbox → Daily → Prod)
const QUOTA_API_ENDPOINTS: [&str; 3] = [
    "https://daily-cloudcode-pa.sandbox.googleapis.com/v1internal:fetchAvailableModels",
    "https://daily-cloudcode-pa.googleapis.com/v1internal:fetchAvailableModels",
    "https://cloudcode-pa.googleapis.com/v1internal:fetchAvailableModels",
];



/// Get shared HTTP Client (15s timeout) for pure info fetching (No JA3)
async fn create_standard_client(_account_id: Option<&str>) -> reqwest::Client {
    crate::utils::http::get_standard_client()
}

/// Get shared HTTP Client (60s timeout) for pure info fetching (No JA3)
#[allow(dead_code)] // 预留给预热/后台任务调用
async fn create_long_standard_client(_account_id: Option<&str>) -> reqwest::Client {
    crate::utils::http::get_long_standard_client()
}

const CLOUD_CODE_BASE_URL: &str = "https://daily-cloudcode-pa.sandbox.googleapis.com";

/// Fetch project ID and subscription tier
async fn fetch_project_id(
    access_token: &str,
    email: &str,
    account_id: Option<&str>,
) -> (Option<String>, Option<String>) {
    let client = create_standard_client(account_id).await;
    let meta = json!({"metadata": {"ideType": "ANTIGRAVITY"}});

    let res = client
        .post(format!("{}/v1internal:loadCodeAssist", CLOUD_CODE_BASE_URL))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(
            reqwest::header::USER_AGENT,
            crate::constants::NATIVE_OAUTH_USER_AGENT,
        )
        .json(&meta)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                if let Ok(data) = res.json::<LoadProjectResponse>().await {
                    let project_id = data.project_id.clone();

                    // Core logic: Multi-level fallback for tier extraction
                    // 1. Paid Tier (Google One AI Premium etc.)
                    // 2. Current Tier (If not ineligible)
                    // 3. Allowed Tiers (Restricted/Default proxy access)
                    let mut subscription_tier = data
                        .paid_tier
                        .as_ref()
                        .and_then(|t| t.name.clone())
                        .or_else(|| data.paid_tier.as_ref().and_then(|t| t.id.clone()));

                    let is_ineligible = data.ineligible_tiers.is_some()
                        && !data.ineligible_tiers.as_ref().unwrap().is_empty();

                    if subscription_tier.is_none() {
                        if !is_ineligible {
                            subscription_tier = data
                                .current_tier
                                .as_ref()
                                .and_then(|t| t.name.clone())
                                .or_else(|| data.current_tier.as_ref().and_then(|t| t.id.clone()));
                        } else {
                            // If account is marked as INELIGIBLE, drop to allowedTiers and extract default
                            if let Some(mut allowed) = data.allowed_tiers {
                                if let Some(default_tier) =
                                    allowed.iter_mut().find(|t| t.is_default == Some(true))
                                {
                                    if let Some(name) = &default_tier.name {
                                        subscription_tier = Some(format!("{} (Restricted)", name));
                                    } else if let Some(id) = &default_tier.id {
                                        subscription_tier = Some(format!("{} (Restricted)", id));
                                    }
                                }
                            }
                        }
                    }

                    if let Some(ref tier) = subscription_tier {
                        crate::modules::logger::log_info(&format!(
                            "📊 [{}] Subscription identified successfully: {}",
                            email, tier
                        ));
                    }

                    return (project_id, subscription_tier);
                }
            } else {
                crate::modules::logger::log_warn(&format!(
                    "⚠️  [{}] loadCodeAssist failed: Status: {}",
                    email,
                    res.status()
                ));
            }
        }
        Err(e) => {
            crate::modules::logger::log_error(&format!(
                "❌ [{}] loadCodeAssist network error: {}",
                email, e
            ));
        }
    }

    (None, None)
}

/// Unified entry point for fetching account quota
pub async fn fetch_quota(
    access_token: &str,
    email: &str,
    account_id: Option<&str>,
) -> crate::error::AppResult<(QuotaData, Option<String>)> {
    fetch_quota_with_cache(access_token, email, None, account_id).await
}

/// Fetch quota with cache support
pub async fn fetch_quota_with_cache(
    access_token: &str,
    email: &str,
    cached_project_id: Option<&str>,
    account_id: Option<&str>,
) -> crate::error::AppResult<(QuotaData, Option<String>)> {
    use crate::error::AppError;

    // Optimization: Skip loadCodeAssist call if project_id is cached to save API quota
    let (project_id, subscription_tier) = if let Some(pid) = cached_project_id {
        (Some(pid.to_string()), None)
    } else {
        fetch_project_id(access_token, email, account_id).await
    };

    // We keep project_id to store in the DB, but we NO LONGER force inject it into payload if it's absent

    let client = create_standard_client(account_id).await;
    let payload = if let Some(ref pid) = project_id {
        json!({ "project": pid })
    } else {
        json!({}) // Empty payload fallback
    };

    let mut last_error: Option<AppError> = None;

    for (ep_idx, ep_url) in QUOTA_API_ENDPOINTS.iter().enumerate() {
        let has_next = ep_idx + 1 < QUOTA_API_ENDPOINTS.len();

        let mut current_payload = payload.clone();
        let mut retry_without_project = false;

        loop {
            match client
                .post(*ep_url)
                .bearer_auth(access_token)
                .header(
                    reqwest::header::USER_AGENT,
                    crate::constants::NATIVE_OAUTH_USER_AGENT,
                )
                .json(&current_payload)
                .send()
                .await
            {
                Ok(response) => {
                    // Convert HTTP error status to AppError
                    if let Err(_) = response.error_for_status_ref() {
                        let status = response.status();
                        let text = response.text().await.unwrap_or_default();

                        #[cfg(debug_assertions)]
                        {
                            crate::modules::logger::log_error(&format!("API Error [{}]: {}", status, text));
                        }

                        // 403 Forbidden 处理：如果是带有 project_id 的请求，尝试剥离后重试
                        if status == reqwest::StatusCode::FORBIDDEN {
                            if current_payload.get("project").is_some() && !retry_without_project {
                                crate::modules::logger::log_warn(&format!(
                                    "Quota fetch got 403 with project ID, retrying without project ID..."
                                ));
                                current_payload = json!({});
                                retry_without_project = true;
                                continue;
                            }

                            // If we still get 403, and there are more endpoints to try, fallback to the next endpoint!
                            if has_next {
                                crate::modules::logger::log_warn(&format!(
                                    "Quota API {} returned 403, falling back to next endpoint",
                                    ep_url
                                ));
                                last_error = Some(AppError::Unknown(format!("HTTP 403 - {}", text)));
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                break; // Break the inner retry loop, continue to next endpoint
                            }

                            crate::modules::logger::log_warn(&format!(
                                "Account unauthorized (403 Forbidden) on all endpoints, marking as forbidden"
                            ));
                            let mut q = QuotaData::new();
                            q.is_forbidden = true;
                            q.subscription_tier = subscription_tier.clone();
                            return Ok((q, project_id.clone()));
                        }

                        // 429/5xx: fallback to next endpoint
                        if has_next
                            && (status == reqwest::StatusCode::TOO_MANY_REQUESTS
                                || status.is_server_error())
                        {
                            crate::modules::logger::log_warn(&format!(
                                "Quota API {} returned {}, falling back to next endpoint",
                                ep_url, status
                            ));
                            last_error =
                                Some(AppError::Unknown(format!("HTTP {} - {}", status, text)));
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            break; // Break the inner retry loop, continue to next endpoint
                        }

                        return Err(AppError::Unknown(format!(
                            "API Error: {} - {}",
                            status, text
                        )));
                    }

                    if ep_idx > 0 {
                        crate::modules::logger::log_info(&format!(
                            "Quota API fallback succeeded at endpoint #{}",
                            ep_idx + 1
                        ));
                    }

                    let quota_response: QuotaResponse =
                        response.json().await.map_err(AppError::from)?;

                    let mut quota_data = QuotaData::new();

                    // Use debug level for detailed info to avoid console noise
                    tracing::debug!("Quota API returned {} models", quota_response.models.len());

                    for (name, info) in quota_response.models {
                        if let Some(quota_info) = info.quota_info {
                            let percentage = quota_info
                                .remaining_fraction
                                .map(|f| (f * 100.0) as i32)
                                .unwrap_or(0);

                            let reset_time = quota_info.reset_time.clone().unwrap_or_default();

                            // Only keep models we care about (exclude internal chat models)
                            if name.starts_with("gemini")
                                || name.starts_with("claude")
                                || name.starts_with("gpt")
                                || name.starts_with("image")
                                || name.starts_with("imagen")
                            {
                                let model_quota = crate::models::quota::ModelQuota {
                                    name,
                                    percentage,
                                    reset_time,
                                    display_name: info.display_name,
                                    supports_images: info.supports_images,
                                    supports_thinking: info.supports_thinking,
                                    thinking_budget: info.thinking_budget,
                                    recommended: info.recommended,
                                    max_tokens: info.max_tokens,
                                    max_output_tokens: info.max_output_tokens,
                                    supported_mime_types: info.supported_mime_types,
                                };
                                quota_data.add_model(model_quota);
                            }
                        }
                    }

                    // Parse deprecated model routing rules
                    if let Some(deprecated) = quota_response.deprecated_model_ids {
                        for (old_id, info) in deprecated {
                            quota_data
                                .model_forwarding_rules
                                .insert(old_id, info.new_model_id);
                        }
                    }

                    // Set subscription tier
                    quota_data.subscription_tier = subscription_tier.clone();

                    return Ok((quota_data, project_id.clone()));
                }
                Err(e) => {
                    crate::modules::logger::log_warn(&format!(
                        "Quota API request failed at {}: {}",
                        ep_url, e
                    ));
                    last_error = Some(AppError::from(e));
                    if has_next {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                    break; // Break the inner retry loop on network error, continue to next endpoint
                }
            }
        } // End of inner loop
    }

    Err(last_error.unwrap_or_else(|| {
        AppError::Unknown("Quota fetch failed: all endpoints exhausted".to_string())
    }))
}

/// Internal fetch quota logic
#[allow(dead_code)]
pub async fn fetch_quota_inner(
    access_token: &str,
    email: &str,
) -> crate::error::AppResult<(QuotaData, Option<String>)> {
    fetch_quota_with_cache(access_token, email, None, None).await
}

/// Batch fetch all account quotas (backup functionality)
#[allow(dead_code)]
pub async fn fetch_all_quotas(
    accounts: Vec<(String, String, String)>,
) -> Vec<(String, crate::error::AppResult<QuotaData>)> {
    let mut results = Vec::new();
    for (id, email, access_token) in accounts {
        let res = fetch_quota(&access_token, &email, Some(&id)).await;
        results.push((email, res.map(|(q, _)| q)));
    }
    results
}

