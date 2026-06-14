# Contributing to Llm Quota

First off, thank you for considering contributing to Llm Quota! It's people like you that make this tool great.

## AI-Assisted Contributions (The `skills/` Folder)

This repository is designed to be highly friendly to AI coding assistants (like Cursor, GitHub Copilot, or Antigravity). 

Before you start coding, please check the [`skills/`](./skills) directory! We have written step-by-step, machine-readable guides for common tasks.
You can simply tell your AI assistant:
> "Read `skills/add-new-api-provider/SKILL.md` and add support for OpenAI."

## Local Development Setup

Llm Quota uses a Rust (Tauri) backend and a React/TypeScript frontend.

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/installation) (`npm install -g pnpm`)

### Installation & Running

1. Clone the repo and install dependencies:
   ```bash
   git clone https://github.com/theasmat/Llm-Quota.git
   cd Llm-Quota
   pnpm install
   ```
2. Start the development server (spawns both Vite and Tauri):
   ```bash
   pnpm tauri dev
   ```

## Pull Request Process

1. **Fork** the repository and create your branch from `master`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. **Conventional Commits**: We enforce conventional commits. Your commit messages (and PR title) must follow this format so our auto-changelog works:
   - `feat: add mistral support`
   - `fix: correct layout overflow on small screens`
   - `docs: update readme`
5. Ensure the test suite passes locally.
6. Issue that pull request!

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md). **Do not use or modify this project to bypass provider Terms of Service.**
