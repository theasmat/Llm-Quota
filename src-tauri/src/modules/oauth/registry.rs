use serde::{Deserialize, Serialize};

pub const CLIENT_ID: &str = "YOUR_GOOGLE_OAUTH_CLIENT_ID";
pub const CLIENT_SECRET: &str = "YOUR_GOOGLE_OAUTH_CLIENT_SECRET";
#[derive(Debug, Clone)]
pub struct OAuthClientConfig {
    pub key: String,
    pub label: String,
    pub client_id: String,
    pub client_secret: String,
    pub is_builtin: bool,
}

#[derive(Debug, Clone)]
struct OAuthClientRegistry {
    clients: Vec<OAuthClientConfig>,
    active_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientDescriptor {
    pub key: String,
    pub label: String,
    pub client_id: String,
    pub is_active: bool,
    pub is_builtin: bool,
}

const OAUTH_CLIENTS_ENV: &str = "ANTIGRAVITY_OAUTH_CLIENTS";
const ACTIVE_OAUTH_CLIENT_ENV: &str = "ANTIGRAVITY_OAUTH_CLIENT_KEY";
const DEFAULT_OAUTH_CLIENT_KEY: &str = "antigravity_enterprise";

static OAUTH_CLIENT_REGISTRY: std::sync::OnceLock<std::sync::RwLock<OAuthClientRegistry>> =
    std::sync::OnceLock::new();

fn normalize_client_key(key: &str) -> String {
    key.trim().to_ascii_lowercase()
}

fn build_registry() -> OAuthClientRegistry {
    let mut clients: Vec<OAuthClientConfig> = vec![OAuthClientConfig {
        key: normalize_client_key(DEFAULT_OAUTH_CLIENT_KEY),
        label: "Antigravity Enterprise".to_string(),
        client_id: CLIENT_ID.to_string(),
        client_secret: CLIENT_SECRET.to_string(),
        is_builtin: true,
    }];

    if let Ok(raw_extra_clients) = std::env::var(OAUTH_CLIENTS_ENV) {
        for entry in raw_extra_clients.split(';') {
            let trimmed = entry.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Expected format: key|client_id|client_secret|optional_label
            let parts: Vec<&str> = trimmed.split('|').map(|v| v.trim()).collect();
            if parts.len() < 3 {
                crate::modules::logger::log_warn(&format!(
                    "Ignored invalid OAuth client entry in {}: {}",
                    OAUTH_CLIENTS_ENV, trimmed
                ));
                continue;
            }

            let key = normalize_client_key(parts[0]);
            if key.is_empty() || parts[1].is_empty() || parts[2].is_empty() {
                crate::modules::logger::log_warn(&format!(
                    "Ignored incomplete OAuth client entry in {}: {}",
                    OAUTH_CLIENTS_ENV, trimmed
                ));
                continue;
            }

            let label = if parts.len() >= 4 && !parts[3].is_empty() {
                parts[3].to_string()
            } else {
                key.clone()
            };

            let custom_client = OAuthClientConfig {
                key: key.clone(),
                label,
                client_id: parts[1].to_string(),
                client_secret: parts[2].to_string(),
                is_builtin: false,
            };

            if let Some(existing_index) = clients.iter().position(|c| c.key == key) {
                clients[existing_index] = custom_client;
                crate::modules::logger::log_info(&format!(
                    "OAuth client '{}' overridden by {}",
                    key, OAUTH_CLIENTS_ENV
                ));
            } else {
                clients.push(custom_client);
                crate::modules::logger::log_info(&format!(
                    "OAuth client '{}' loaded from {}",
                    key, OAUTH_CLIENTS_ENV
                ));
            }
        }
    }

    let mut active_key = std::env::var(ACTIVE_OAUTH_CLIENT_ENV)
        .ok()
        .map(|v| normalize_client_key(&v))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| normalize_client_key(DEFAULT_OAUTH_CLIENT_KEY));

    if !clients.iter().any(|c| c.key == active_key) {
        active_key = clients
            .first()
            .map(|c| c.key.clone())
            .unwrap_or_else(|| normalize_client_key(DEFAULT_OAUTH_CLIENT_KEY));
    }

    OAuthClientRegistry {
        clients,
        active_key,
    }
}

fn oauth_registry() -> &'static std::sync::RwLock<OAuthClientRegistry> {
    OAUTH_CLIENT_REGISTRY.get_or_init(|| std::sync::RwLock::new(build_registry()))
}

fn get_client_by_key<'a>(
    clients: &'a [OAuthClientConfig],
    client_key: &str,
) -> Option<&'a OAuthClientConfig> {
    let normalized = normalize_client_key(client_key);
    clients.iter().find(|c| c.key == normalized)
}

fn active_or_first_client(registry: &OAuthClientRegistry) -> Option<OAuthClientConfig> {
    if let Some(active) = get_client_by_key(&registry.clients, &registry.active_key) {
        return Some(active.clone());
    }
    registry.clients.first().cloned()
}

pub fn select_auth_client(client_key: Option<&str>) -> Result<OAuthClientConfig, String> {
    let registry_guard = oauth_registry().read().map_err(|e| e.to_string())?;
    let registry = &*registry_guard;

    if registry.clients.is_empty() {
        return Err("No OAuth clients configured".to_string());
    }

    if let Some(key) = client_key {
        if let Some(client) = get_client_by_key(&registry.clients, key) {
            return Ok(client.clone());
        }
        return Err(format!("Unknown OAuth client key: {}", key));
    }

    active_or_first_client(registry).ok_or_else(|| "No OAuth clients configured".to_string())
}

pub fn get_candidate_clients(preferred_client_key: Option<&str>) -> Vec<OAuthClientConfig> {
    let registry_guard = match oauth_registry().read() {
        Ok(guard) => guard,
        Err(_) => return vec![],
    };
    let registry = &*registry_guard;

    let mut candidates = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let mut push_candidate = |client: &OAuthClientConfig| {
        if seen.insert(client.key.clone()) {
            candidates.push(client.clone());
        }
    };

    if let Some(preferred_key) = preferred_client_key {
        if let Some(preferred) = get_client_by_key(&registry.clients, preferred_key) {
            push_candidate(preferred);
        } else {
            crate::modules::logger::log_warn(&format!(
                "Preferred OAuth client '{}' not found; fallback to active client list",
                preferred_key
            ));
        }
    }

    if let Some(active) = get_client_by_key(&registry.clients, &registry.active_key) {
        push_candidate(active);
    }

    for client in &registry.clients {
        push_candidate(client);
    }

    candidates
}

pub fn is_client_mismatch_error(status: reqwest::StatusCode, error_text: &str) -> bool {
    let text = error_text.to_ascii_lowercase();
    status == reqwest::StatusCode::BAD_REQUEST
        || status == reqwest::StatusCode::UNAUTHORIZED
        || status == reqwest::StatusCode::FORBIDDEN
        || text.contains("unauthorized_client")
        || text.contains("invalid_client")
}

pub fn normalize_refreshed_oauth_client_key(
    current_token: &crate::models::TokenData,
    refreshed_client_key: Option<String>,
) -> Option<String> {
    let resolved = refreshed_client_key.or_else(|| current_token.oauth_client_key.clone());
    let project_missing = current_token
        .project_id
        .as_deref()
        .map(str::trim)
        .map(|value| value.is_empty())
        .unwrap_or(true);

    if current_token.oauth_client_key.is_none()
        && project_missing
        && matches!(resolved.as_deref(), Some("antigravity_enterprise"))
    {
        crate::modules::logger::log_warn(
            "Refreshed token via enterprise client for a legacy account without project_id; keep oauth_client_key unset to avoid accidental enterprise lock",
        );
        return None;
    }

    resolved
}

pub fn list_oauth_clients() -> Result<Vec<OAuthClientDescriptor>, String> {
    let registry_guard = oauth_registry().read().map_err(|e| e.to_string())?;
    let registry = &*registry_guard;

    Ok(registry
        .clients
        .iter()
        .map(|client| OAuthClientDescriptor {
            key: client.key.clone(),
            label: client.label.clone(),
            client_id: client.client_id.clone(),
            is_active: client.key == registry.active_key,
            is_builtin: client.is_builtin,
        })
        .collect())
}

pub fn get_active_oauth_client_key() -> Result<String, String> {
    let registry_guard = oauth_registry().read().map_err(|e| e.to_string())?;
    Ok(registry_guard.active_key.clone())
}

pub fn set_active_oauth_client_key(client_key: &str) -> Result<(), String> {
    let mut registry_guard = oauth_registry().write().map_err(|e| e.to_string())?;
    let normalized = normalize_client_key(client_key);

    if get_client_by_key(&registry_guard.clients, &normalized).is_none() {
        let available = registry_guard
            .clients
            .iter()
            .map(|c| c.key.clone())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "Unknown OAuth client key '{}'. Available: {}",
            client_key, available
        ));
    }

    registry_guard.active_key = normalized.clone();
    crate::modules::logger::log_info(&format!("Active OAuth client switched to '{}'", normalized));
    Ok(())
}
