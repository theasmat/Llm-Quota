<div align="center">
  <img src="assets/logo/icon.png" width="128" alt="Llm Quota Logo"/>
  <h1>Llm Quota</h1>
  <p>A compact, high-density dashboard to manage quota limits for LLM accounts.</p>

  <a href="https://github.com/theasmat/llm-quota/releases/latest">
    <img src="https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white" alt="macOS" />
  </a>
  <a href="https://github.com/theasmat/llm-quota/releases/latest">
    <img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="Windows" />
  </a>
  <a href="https://github.com/theasmat/llm-quota/releases/latest">
    <img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt="Linux" />
  </a>
</div>

<br/>

Llm Quota is a compact, high-density dashboard built with Tauri and React to manage and monitor quota limits for LLM accounts like Google Gemini. Designed to look and feel like a professional spreadsheet-style internal tool, it offers a fast, local-first experience with a dense data grid, dark/light theme support, and a highly responsive native UI.

## 🚀 Downloads

| OS | Download Link | Notes |
|----|--------------|-------|
| <img src="https://cdn.simpleicons.org/apple/black" width="16" /> **macOS** | [Download .dmg](https://github.com/theasmat/llm-quota/releases/latest) | Or use Homebrew: `brew install --no-quarantine theasmat/llm-quota/llm-quota` |
| <img src="https://cdn.simpleicons.org/windows/0078D6" width="16" /> **Windows** | [Download .exe](https://github.com/theasmat/llm-quota/releases/latest) | Standalone Windows installer |
| <img src="https://cdn.simpleicons.org/linux/black" width="16" /> **Linux** | [Download .AppImage / .deb](https://github.com/theasmat/llm-quota/releases/latest) | Universal AppImage or native packages |

<br/>

## ✨ Features

- ⚡ **Local-First & Fast**: Built with Tauri and Rust for a lightweight, deeply integrated native desktop experience.
- 📊 **High-Density Dashboard**: Ultra-compact UI designed to maximize screen real estate and give you an overview of your quotas at a glance.
- 🔑 **Account Management**: Add, track, and monitor API keys and usage quotas securely on your local machine.
- 🎨 **Built-in Themes**: Natively supports both Light and Dark mode with seamless Tailwind CSS integration.
- 📥 **Export & Backup**: Export your account data easily for safekeeping.

## 🛠️ Tech Stack

- **Frontend**: React, TypeScript, Tailwind CSS, Lucide React
- **Backend**: Tauri, Rust
- **State Management**: Zustand
- **Icons**: Lucide

## 💻 Installation

### macOS (Homebrew)
The easiest way to install on macOS is using our Homebrew tap:
```bash
brew install --no-quarantine theasmat/llm-quota/llm-quota
```

### macOS & Linux (Install Script)
You can use the automated install script:
```bash
curl -fsSL https://raw.githubusercontent.com/theasmat/llm-quota/main/install.sh | bash
```

### Windows
Download the `.exe` installer from the [Releases](https://github.com/theasmat/llm-quota/releases/latest) page.

---

## Setup & Development

### Prerequisites

Ensure you have the following installed on your system:
- [Node.js](https://nodejs.org/) (v16+)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)

### Local Build

1. Clone the repository and navigate to the project directory:
   ```bash
   git clone https://github.com/theasmat/llm-quota.git
   cd llm-quota
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Run the development server:
   ```bash
   pnpm tauri dev
   ```

### Build for Production

To build the application for your operating system:

```bash
pnpm tauri build
```

The compiled binaries will be available in the `src-tauri/target/release` directory.

## License

This project is licensed under the [MIT License](LICENSE).
