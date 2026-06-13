use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme: Theme,
    #[serde(default)]
    pub language: String,
    #[serde(default = "default_true")]
    pub auto_check_updates: bool,
}

fn default_true() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en-US".to_string(),
            auto_check_updates: true,
        }
    }
}
