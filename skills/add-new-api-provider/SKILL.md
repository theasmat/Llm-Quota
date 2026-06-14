---
name: add-new-api-provider
description: >
  Step-by-step guide for adding a new AI model provider (e.g., OpenAI, Mistral)
  to the quota tracking system. Covers backend API module changes, model filtering,
  response type mapping, and frontend display.
  Trigger: "add provider", "add new model", "track new API", "add quota source",
  "support OpenAI", "support Mistral", "new quota endpoint".
---

# How to Add a New AI Provider

This guide walks through adding a new provider's quota endpoint so Llm Quota can track it alongside existing Gemini/Claude/GPT quotas.

---

## Understanding the Current Architecture

Quota fetching lives in one file: `src-tauri/src/modules/quota/api.rs`.

Key items:
- `QUOTA_API_ENDPOINTS` — ordered list of Google quota API URLs (fallback chain)
- `fetch_quota_with_cache()` — main function; iterates endpoints, parses `QuotaResponse`
- Model filtering is done at line ~263: only `gemini`, `claude`, `gpt`, `image`, `imagen` prefixes are kept

The current system is **Google-centric** (one unified API returns quotas for all models).  
If adding a **separate, independent provider API** (e.g., OpenAI's own usage API), you will need a new module.

---

## Scenario A: New Model Name Prefix (Same Google API)

If the new model is already returned by the existing Google quota API but just has a different name prefix (e.g., `llama-*` or `deepseek-*`):

### Step 1 — Add the prefix filter in `api.rs`

File: `src-tauri/src/modules/quota/api.rs` around line 263:

```rust
// BEFORE
if name.starts_with("gemini")
    || name.starts_with("claude")
    || name.starts_with("gpt")
    || name.starts_with("image")
    || name.starts_with("imagen")

// AFTER (add your prefix)
if name.starts_with("gemini")
    || name.starts_with("claude")
    || name.starts_with("gpt")
    || name.starts_with("image")
    || name.starts_with("imagen")
    || name.starts_with("llama")     // ← add here
```

That's it for Scenario A. The rest of the pipeline handles it automatically.

---

## Scenario B: Entirely New Provider with Its Own API

For a provider with its own separate API endpoint (e.g., OpenAI usage API):

### Step 1 — Create a new module

Create: `src-tauri/src/modules/openai/mod.rs`  
Create: `src-tauri/src/modules/openai/api.rs`

In `api.rs`, implement a function with this signature:
```rust
pub async fn fetch_openai_quota(
    api_key: &str,
    account_id: Option<&str>,
) -> crate::error::AppResult<crate::models::QuotaData> {
    // 1. Build HTTP client via crate::utils::http::get_standard_client()
    // 2. Call provider's usage/quota API
    // 3. Map response into QuotaData + ModelQuota structs
    // 4. Return Ok(quota_data)
}
```

### Step 2 — Register the module

In `src-tauri/src/modules/mod.rs`, add:
```rust
pub mod openai;
```

### Step 3 — Add a model to store the API key

Extend the `Account` model (`src-tauri/src/models/account.rs`) to add an optional `openai_api_key: Option<String>` field if needed.

### Step 4 — Expose a new Tauri command

In `src-tauri/src/commands/account.rs`:
```rust
#[tauri::command]
pub async fn fetch_openai_quota(account_id: String) -> Result<QuotaData, String> {
    // Load account/api_key from state
    // Call modules::openai::api::fetch_openai_quota(...)
    // Return result
}
```

In `src-tauri/src/commands/mod.rs`, add it to `tauri::generate_handler![]`.

### Step 5 — Map the response to `QuotaData`

`QuotaData` is defined in `src-tauri/src/models/quota.rs`.  
Use `quota_data.add_model(ModelQuota { name, percentage, reset_time, ... })` to populate it.

### Step 6 — Call from frontend

In the relevant frontend service file (`src/services/`):
```typescript
import { invoke } from '@tauri-apps/api/core';

export const fetchOpenAIQuota = (accountId: string) =>
  invoke<QuotaData>('fetch_openai_quota', { accountId });
```

---

## Checklist

- [ ] Added/confirmed model name prefix filter in `api.rs`
- [ ] (If new API) Created `modules/<provider>/api.rs` with proper `AppResult` return type
- [ ] (If new API) Registered module in `modules/mod.rs`
- [ ] (If new API) Added Tauri command and registered in `commands/mod.rs`
- [ ] Populated `QuotaData` + `ModelQuota` structs from the API response
- [ ] Called the command from frontend service
- [ ] Tested with `DRY_RUN` / mock data to verify data flows to UI
- [ ] Added translation key in `src/locales/` for any new UI label
