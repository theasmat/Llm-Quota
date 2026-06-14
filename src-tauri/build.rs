use std::fs;
use std::path::Path;

fn main() {
    // Automatically load variables from ../.env and expose them to option_env!
    let env_path = Path::new("../.env");
    // Always tell cargo to re-run if .env changes, even if it doesn't exist yet!
    println!("cargo:rerun-if-changed=../.env");
    if env_path.exists() {
        if let Ok(content) = fs::read_to_string(env_path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim().trim_matches('"').trim_matches('\'');
                    println!("cargo:rustc-env={}={}", key, value);
                }
            }
        }
    }

    tauri_build::build()
}
