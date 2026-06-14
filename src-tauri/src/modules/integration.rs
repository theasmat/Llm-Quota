use std::path::PathBuf;
use std::fs;
use serde_json::Value;

fn get_home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default()
}

fn get_ide_db_path() -> Option<PathBuf> {
    let home = get_home_dir();
    #[cfg(target_os = "macos")]
    return Some(home.join("Library/Application Support/Antigravity IDE/User/globalStorage/state.vscdb"));
    
    #[cfg(target_os = "windows")]
    if let Ok(appdata) = std::env::var("APPDATA") {
        return Some(PathBuf::from(appdata).join("Antigravity IDE").join("User").join("globalStorage").join("state.vscdb"));
    }
    
    #[cfg(target_os = "linux")]
    return Some(home.join(".config/Antigravity IDE/User/globalStorage/state.vscdb"));
    
    #[allow(unreachable_code)]
    None
}

pub fn check_local_integrations(email: &str) -> Vec<String> {
    let mut integrations = Vec::new();
    let email_lower = email.to_lowercase();

    // 1. Antigravity 2.0
    // Check ~/.gemini/antigravity/accounts.json AND ~/.antigravity_tools/accounts.json
    let mut is_ag_2_0 = false;
    let ag_path = get_home_dir().join(".gemini").join("antigravity").join("accounts.json");
    if let Ok(content) = fs::read_to_string(&ag_path) {
        if content.to_lowercase().contains(&email_lower) {
            is_ag_2_0 = true;
        }
    }
    let ag_tools_path = get_home_dir().join(".antigravity_tools").join("accounts.json");
    if let Ok(content) = fs::read_to_string(&ag_tools_path) {
        if content.to_lowercase().contains(&email_lower) {
            is_ag_2_0 = true;
        }
    }
    if is_ag_2_0 {
        integrations.push("Antigravity 2.0".to_string());
    }

    // 2. Antigravity IDE
    // Check globalStorage/state.vscdb
    if let Some(vscdb_path) = get_ide_db_path() {
        if vscdb_path.exists() {
            if let Ok(output) = std::process::Command::new("sqlite3")
                .arg(&vscdb_path)
                .arg("SELECT value FROM ItemTable WHERE key LIKE '%oauth%' OR key LIKE '%userStatus%' OR key LIKE '%agentManagerInitState%';")
                .output()
            {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    use base64::{engine::general_purpose, Engine as _};
                    for line in stdout.lines() {
                        let trimmed = line.trim();
                        if trimmed.is_empty() { continue; }
                        // Some values might be prefixed with quotes or spaces, but normally just base64
                        if let Ok(decoded) = general_purpose::STANDARD.decode(trimmed) {
                            let decoded_str = String::from_utf8_lossy(&decoded).to_lowercase();
                            if decoded_str.contains(&email_lower) {
                                integrations.push("Antigravity IDE".to_string());
                                break; // found it, no need to check other rows
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. AGCLI
    // Check ~/.gemini/google_accounts.json
    let agcli_path = get_home_dir().join(".gemini").join("google_accounts.json");
    if let Ok(content) = fs::read_to_string(&agcli_path) {
        if content.to_lowercase().contains(&email_lower) {
            integrations.push("AGCLI".to_string());
        }
    }

    // 4. Gemini
    // Check ~/.gemini/oauth_creds.json
    let gemini_path = get_home_dir().join(".gemini").join("oauth_creds.json");
    if let Ok(content) = fs::read_to_string(&gemini_path) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(id_token) = json.get("id_token").and_then(|v| v.as_str()) {
                let parts: Vec<&str> = id_token.split('.').collect();
                if parts.len() >= 2 {
                    use base64::{engine::general_purpose, Engine as _};
                    if let Ok(decoded) = general_purpose::URL_SAFE_NO_PAD.decode(parts[1]) {
                        if let Ok(payload_str) = String::from_utf8(decoded) {
                            if payload_str.to_lowercase().contains(&email_lower) {
                                integrations.push("Gemini".to_string());
                            }
                        }
                    } else if let Ok(decoded) = general_purpose::STANDARD.decode(parts[1]) {
                        if let Ok(payload_str) = String::from_utf8(decoded) {
                            if payload_str.to_lowercase().contains(&email_lower) {
                                integrations.push("Gemini".to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut unique = integrations;
    unique.sort();
    unique.dedup();
    unique
}
