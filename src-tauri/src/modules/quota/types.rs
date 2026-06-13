use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaResponse {
    pub models: std::collections::HashMap<String, ModelInfo>,
    #[serde(rename = "deprecatedModelIds")]
    pub deprecated_model_ids: Option<std::collections::HashMap<String, DeprecatedModelInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecatedModelInfo {
    #[serde(rename = "newModelId")]
    pub new_model_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    #[serde(rename = "quotaInfo")]
    pub quota_info: Option<QuotaInfo>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "supportsImages")]
    pub supports_images: Option<bool>,
    #[serde(rename = "supportsThinking")]
    pub supports_thinking: Option<bool>,
    #[serde(rename = "thinkingBudget")]
    pub thinking_budget: Option<i32>,
    pub recommended: Option<bool>,
    #[serde(rename = "maxTokens")]
    pub max_tokens: Option<i32>,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: Option<i32>,
    #[serde(rename = "supportedMimeTypes")]
    pub supported_mime_types: Option<std::collections::HashMap<String, bool>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaInfo {
    #[serde(rename = "remainingFraction")]
    pub remaining_fraction: Option<f64>,
    #[serde(rename = "resetTime")]
    pub reset_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoadProjectResponse {
    #[serde(rename = "cloudaicompanionProject")]
    pub project_id: Option<String>,
    #[serde(rename = "currentTier")]
    pub current_tier: Option<Tier>,
    #[serde(rename = "paidTier")]
    pub paid_tier: Option<Tier>,
    #[serde(rename = "allowedTiers")]
    pub allowed_tiers: Option<Vec<Tier>>,
    #[serde(rename = "ineligibleTiers")]
    pub ineligible_tiers: Option<Vec<IneligibleTier>>,
}

#[derive(Debug, Deserialize)]
pub struct IneligibleTier {
    #[allow(dead_code)]
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tier {
    #[allow(dead_code)]
    pub is_default: Option<bool>,
    pub id: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "quotaTier")]
    pub quota_tier: Option<String>,
    pub name: Option<String>,
    #[allow(dead_code)]
    pub slug: Option<String>,
}
