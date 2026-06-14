---
name: project-overview
description: >
  Complete architectural overview of Llm Quota for new contributors.
  Covers the tech stack (Tauri + React/TypeScript), directory map,
  and how the Rust backend and React frontend communicate.
  Read this FIRST before any other skill in this folder.
  Trigger: "explain the project", "where does X live", "project structure",
  "what is this codebase", "new contributor".
---

# Llm Quota — Project Overview

Llm Quota is a cross-platform desktop app (macOS, Linux, Windows) built with **Tauri**.  
It tracks AI model quota usage across multiple Google accounts simultaneously.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Backend (Rust) | Tauri 2 + Tokio async runtime + reqwest HTTP client |
| Frontend (UI) | React 18 + TypeScript + TailwindCSS + Zustand state |
| Bundler | Vite |
| IPC Bridge | Tauri `invoke()` — frontend calls Rust commands by name |
| Auth | OAuth 2.0 (Google) — native browser flow |
| Local Storage | Tauri file-system APIs (JSON config files) |
| CI/CD | GitHub Actions — builds multi-arch binaries on every tag push |

---

## Directory Map

```
Llm-Quota/
├── src/                     # React frontend (TypeScript)
│   ├── components/          # UI components (accounts, dashboard, sidebar, etc.)
│   ├── pages/               # Page-level views
│   ├── stores/              # Zustand state stores
│   ├── services/            # Frontend service helpers (call Tauri invoke)
│   ├── locales/             # i18n translation strings (en, zh, etc.)
│   └── types/               # TypeScript type definitions
│
├── src-tauri/               # Rust backend (Tauri)
│   └── src/
│       ├── commands/        # Tauri IPC commands (exposed to frontend)
│       │   ├── account.rs   # Account CRUD, quota fetching
│       │   ├── oauth.rs     # OAuth login flow
│       │   ├── config.rs    # Config read/write
│       │   ├── integration.rs # External integration commands
│       │   └── mod.rs       # Registers all commands in generate_handler![]
│       ├── modules/         # Core business logic (not exposed to frontend)
│       │   ├── quota/       # Quota API fetching logic
│       │   │   ├── api.rs   # HTTP calls to Google quota endpoints
│       │   │   └── types.rs # API response types
│       │   ├── oauth/       # Token refresh, OAuth client management
│       │   ├── account.rs   # Account model & persistence
│       │   ├── cache.rs     # In-memory quota cache
│       │   └── config.rs    # App config management
│       ├── models/          # Shared data models (QuotaData, ModelQuota, etc.)
│       ├── utils/           # Shared utilities (HTTP client, file helpers)
│       ├── error.rs         # AppError enum + AppResult type alias
│       └── lib.rs           # Tauri app entry — registers plugins & invoke_handler
│
├── install/
│   ├── mac.sh               # macOS installer script
│   └── linux.sh             # Linux installer script
├── .github/workflows/
│   └── release.yml          # CI: build → release → update README + install scripts
└── skills/                  # ← You are here — contributor guides
```

---

## Data Flow (How a Feature Works End-to-End)

```
User clicks "Refresh" in UI
        │
        ▼
src/services/*.ts calls:
  invoke("fetch_account_quota", { accountId })
        │
        ▼
src-tauri/src/commands/account.rs
  #[tauri::command] fetch_account_quota(...)
  → calls modules::quota::fetch_quota(...)
        │
        ▼
src-tauri/src/modules/quota/api.rs
  → HTTP POST to Google quota API endpoints
  → Returns QuotaData struct
        │
        ▼
Result serialized to JSON → sent back to frontend
        │
        ▼
Zustand store updated → React re-renders quota display
```

---

## Key Concepts

- **`AppError` / `AppResult`**: All backend errors use `src-tauri/src/error.rs`. Return `AppResult<T>` from commands — Tauri auto-serializes errors as strings.
- **`QuotaData`**: The main model (`src-tauri/src/models/quota.rs`) — contains a list of `ModelQuota` entries, `is_forbidden` flag, and `subscription_tier`.
- **`QUOTA_API_ENDPOINTS`**: Three fallback Google API URLs in `modules/quota/api.rs`. The app tries them in order (sandbox → daily → prod).
- **Caching**: `modules/cache.rs` stores quota data in-memory to avoid hammering Google's API. Cache key is account ID.
- **i18n**: All user-visible text should go through `src/locales/` translation files. Use `useTranslation()` hook in components.

---

## Next Steps

- To add a new AI provider → read `02-add-new-api-provider.md`
- To add a UI component → read `03-add-frontend-component.md`
- To add a backend command → read `04-add-tauri-command.md`
- To understand CI/releases → read `05-release-and-ci.md`
