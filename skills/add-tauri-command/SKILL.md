---
name: add-tauri-command
description: >
  Step-by-step guide for exposing a new Rust backend function as a Tauri IPC
  command callable from the React frontend via invoke(). Covers command declaration,
  registration in mod.rs, error handling with AppError, and the frontend invoke call.
  Trigger: "add backend command", "new tauri command", "expose rust function",
  "add invoke", "backend feature", "add command", "rust endpoint".
---

# How to Add a Tauri Backend Command

Tauri commands are Rust async functions annotated with `#[tauri::command]` that the frontend calls via `invoke()`. This is the IPC bridge between React and Rust.

---

## File Overview

| File | Role |
|---|---|
| `src-tauri/src/commands/account.rs` | Account & quota commands |
| `src-tauri/src/commands/oauth.rs` | OAuth login flow commands |
| `src-tauri/src/commands/config.rs` | Config read/write commands |
| `src-tauri/src/commands/system.rs` | System/window commands |
| `src-tauri/src/commands/integration.rs` | External integration commands |
| `src-tauri/src/commands/mod.rs` | **Registers ALL commands** in `generate_handler![]` |

---

## Step 1 — Write the Command Function

Add your function to the most relevant file in `src-tauri/src/commands/`.  
If it doesn't fit, create a new file (e.g., `src-tauri/src/commands/analytics.rs`).

```rust
use crate::error::AppResult;

#[tauri::command]
pub async fn get_account_stats(account_id: String) -> Result<AccountStats, String> {
    // Call a module function
    let stats = crate::modules::stats::compute_stats(&account_id)
        .await
        .map_err(|e| e.to_string())?;   // ← Convert AppError to String for Tauri

    Ok(stats)
}
```

### Rules:
- Always return `Result<T, String>` — Tauri serializes the `Err(String)` for you.
- Use `crate::error::AppResult<T>` internally in module functions, then `.map_err(|e| e.to_string())?` at the command boundary.
- Input parameters are automatically deserialized from the frontend's `invoke()` args (JSON → Rust types via serde).
- Output is automatically serialized to JSON (Rust → JSON → TypeScript).
- Mark the function `async` even if it doesn't await — Tauri requires it for the handler generator.

---

## Step 2 — Register the Command

Open `src-tauri/src/commands/mod.rs`.

**If using a new file**, add the module declaration at the top:
```rust
pub mod analytics;   // ← add this
use analytics::*;   // ← add this
```

Add the function name to `tauri::generate_handler![]`:
```rust
pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        list_accounts,
        // ... existing commands ...
        get_account_stats,   // ← add here
    ]
}
```

⚠️ If you forget to add it here, the frontend `invoke()` call will return an error saying the command is not found.

---

## Step 3 — Define the Return Type

If returning a custom struct, define it in `src-tauri/src/models/` and derive the required traits:

```rust
// src-tauri/src/models/stats.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStats {
    pub total_requests: u64,
    pub quota_used_percent: f32,
    pub last_refresh: String,
}
```

Register the model in `src-tauri/src/models/mod.rs`:
```rust
pub mod stats;
pub use stats::AccountStats;
```

---

## Step 4 — Call from the Frontend

Create or update a service in `src/services/`:

```typescript
// src/services/analyticsService.ts
import { invoke } from '@tauri-apps/api/core';

export interface AccountStats {
  totalRequests: number;       // Rust snake_case → TS camelCase automatically
  quotaUsedPercent: number;
  lastRefresh: string;
}

export const getAccountStats = (accountId: string): Promise<AccountStats> =>
  invoke<AccountStats>('get_account_stats', { accountId });
```

> **Note:** Tauri automatically converts Rust `snake_case` struct field names to JSON `camelCase` on serialization. Your TypeScript interface should use `camelCase`.

---

## Step 5 — Use in a Component

```tsx
import { getAccountStats } from '@/services/analyticsService';
import { useEffect, useState } from 'react';

const StatsPanel = ({ accountId }: { accountId: string }) => {
  const [stats, setStats] = useState<AccountStats | null>(null);

  useEffect(() => {
    getAccountStats(accountId)
      .then(setStats)
      .catch(console.error);
  }, [accountId]);

  if (!stats) return <div>Loading...</div>;
  return <div>{stats.totalRequests} requests</div>;
};
```

---

## Error Handling Pattern

Use `AppError` from `src-tauri/src/error.rs` inside modules, then convert at the command layer:

```rust
// In a module (returns AppResult)
pub async fn compute_stats(id: &str) -> AppResult<AccountStats> {
    if id.is_empty() {
        return Err(AppError::Unknown("Account ID cannot be empty".to_string()));
    }
    // ...
    Ok(stats)
}

// In the command (converts to String for Tauri)
#[tauri::command]
pub async fn get_account_stats(account_id: String) -> Result<AccountStats, String> {
    crate::modules::stats::compute_stats(&account_id)
        .await
        .map_err(|e| e.to_string())
}
```

---

## Checklist

- [ ] Command function written with `#[tauri::command]` and `async`
- [ ] Returns `Result<T, String>` (not `AppResult`)
- [ ] Module logic kept in `src-tauri/src/modules/` (not in the command file)
- [ ] Return type struct defined in `src-tauri/src/models/` with `Serialize, Deserialize`
- [ ] Model registered in `src-tauri/src/models/mod.rs`
- [ ] Command registered in `src-tauri/src/commands/mod.rs` → `generate_handler![]`
- [ ] Frontend service created in `src/services/` using `invoke()`
- [ ] TypeScript interface uses `camelCase` field names
