# Skills — Contributor Guides

This folder contains **AI-readable skill guides** for contributing to Llm Quota.  
Each file is structured so an AI coding assistant can read it and immediately start adding features.

## Available Skills

| File | What It Covers | Who Should Read It |
|---|---|---|
| [project-overview](./project-overview/SKILL.md) | Full architecture, directory map, data flow | Everyone — read first |
| [add-new-api-provider](./add-new-api-provider/SKILL.md) | Adding a new AI quota source (OpenAI, Mistral, etc.) | Backend contributors |
| [add-frontend-component](./add-frontend-component/SKILL.md) | React components, Zustand stores, i18n | Frontend contributors |
| [add-tauri-command](./add-tauri-command/SKILL.md) | Exposing Rust functions to the frontend via `invoke()` | Full-stack contributors |
| [release-and-ci](./release-and-ci/SKILL.md) | GitHub Actions, multi-platform builds, how to cut a release | DevOps / release managers |
| [install-scripts](./install-scripts/SKILL.md) | mac.sh / linux.sh, dry run testing, adding new platforms | Packaging contributors |

## How to Use These with an AI Assistant

When working with an AI coding assistant (e.g., Antigravity, Cursor, GitHub Copilot), you can say:

> "Read `skills/02-add-new-api-provider.md` and add support for OpenAI quota tracking."

The AI will follow the step-by-step instructions to make the correct changes across the right files.
