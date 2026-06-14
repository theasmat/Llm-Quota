# Skills — Contributor Guides

This folder contains **AI-readable skill guides** for contributing to Llm Quota.  
Each file is structured so an AI coding assistant can read it and immediately start adding features.

## Available Skills

| File | What It Covers | Who Should Read It |
|---|---|---|
| [01-project-overview.md](./01-project-overview.md) | Full architecture, directory map, data flow | Everyone — read first |
| [02-add-new-api-provider.md](./02-add-new-api-provider.md) | Adding a new AI quota source (OpenAI, Mistral, etc.) | Backend contributors |
| [03-add-frontend-component.md](./03-add-frontend-component.md) | React components, Zustand stores, i18n | Frontend contributors |
| [04-add-tauri-command.md](./04-add-tauri-command.md) | Exposing Rust functions to the frontend via `invoke()` | Full-stack contributors |
| [05-release-and-ci.md](./05-release-and-ci.md) | GitHub Actions, multi-platform builds, how to cut a release | DevOps / release managers |
| [06-install-scripts.md](./06-install-scripts.md) | mac.sh / linux.sh, dry run testing, adding new platforms | Packaging contributors |

## How to Use These with an AI Assistant

When working with an AI coding assistant (e.g., Antigravity, Cursor, GitHub Copilot), you can say:

> "Read `skills/02-add-new-api-provider.md` and add support for OpenAI quota tracking."

The AI will follow the step-by-step instructions to make the correct changes across the right files.
