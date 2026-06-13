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
  <br/>
  
  <p>
    <a href="https://github.com/theasmat/llm-quota/stargazers">
      <img src="https://img.shields.io/github/stars/theasmat/llm-quota?style=for-the-badge&color=FCC624&logo=github&logoColor=black" alt="Stars" />
    </a>
    <a href="https://github.com/theasmat/llm-quota/network/members">
      <img src="https://img.shields.io/github/forks/theasmat/llm-quota?style=for-the-badge&color=0078D6&logo=github&logoColor=white" alt="Forks" />
    </a>
    <a href="https://github.com/theasmat/llm-quota/issues">
      <img src="https://img.shields.io/github/issues/theasmat/llm-quota?style=for-the-badge&color=000000&logo=github&logoColor=white" alt="Issues" />
    </a>
    <a href="https://github.com/theasmat/llm-quota/blob/master/LICENSE">
      <img src="https://img.shields.io/github/license/theasmat/llm-quota?style=for-the-badge&color=success&logo=open-source-initiative&logoColor=white" alt="License" />
    </a>
  </p>
</div>

<br/>

Llm Quota is a compact, high-density dashboard built with Tauri and React to manage and monitor quota limits for LLM accounts like Google Gemini. Designed to look and feel like a professional spreadsheet-style internal tool, it offers a fast, local-first experience with a dense data grid, dark/light theme support, and a highly responsive native UI.

<br/>

## 📖 Table of Contents
- [🚀 Downloads](#-downloads)
- [✨ Features](#-features)
- [🛠️ Tech Stack](#️-tech-stack)
- [<img src="https://raw.githubusercontent.com/Tarikul-Islam-Anik/Animated-Fluent-Emojis/master/Emojis/Objects/Laptop.png" alt="Laptop" width="20" height="20" /> Installation](#installation)
- [📷 Screenshots](#-screenshots)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

<br/>

## 🚀 Downloads

| OS | Download Link | Notes |
|----|--------------|-------|
| <img src="https://skillicons.dev/icons?i=apple" width="16" /> **macOS** | [Download .dmg](https://github.com/theasmat/llm-quota/releases/latest) | Or use Homebrew: `brew install --no-quarantine theasmat/llm-quota/llm-quota` |
| <img src="https://skillicons.dev/icons?i=windows" width="16" /> **Windows** | [Download .exe](https://github.com/theasmat/llm-quota/releases/latest) | Standalone Windows installer |
| <img src="https://skillicons.dev/icons?i=linux" width="16" /> **Linux** | [Download .AppImage / .deb](https://github.com/theasmat/llm-quota/releases/latest) | Universal AppImage or native packages |

<br/>

## ✨ Features

- ⚡ **Local-First & Fast**: Built with Tauri and Rust for a lightweight, deeply integrated native desktop experience.
- 📊 **High-Density Dashboard**: Ultra-compact UI designed to maximize screen real estate and give you an overview of your quotas at a glance.
- 🔑 **Account Management**: Add, track, and monitor API keys and usage quotas securely on your local machine.
- 🎨 **Built-in Themes**: Natively supports both Light and Dark mode with seamless Tailwind CSS integration.
- 📥 **Export & Backup**: Export your account data easily for safekeeping.

## 🛠️ Tech Stack

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=react,ts,tailwind,rust,githubactions&theme=light" />
  </a>
</p>

- **Frontend**: React, TypeScript, Tailwind CSS
- **Backend**: Tauri, Rust
- **State Management**: Zustand
- **Icons**: Lucide

## <img src="https://raw.githubusercontent.com/Tarikul-Islam-Anik/Animated-Fluent-Emojis/master/Emojis/Objects/Laptop.png" alt="Laptop" width="30" height="30" /> Installation

<!-- DOWNLOAD_TABLE_START -->
<!-- The download table will be automatically injected here by GitHub Actions -->
| Architecture / OS | 🍎 macOS | 🪟 Windows | 🐧 Linux |
|:---:|:---:|:---:|:---:|
| **x86_64 (Intel/AMD)** | [⬇️ Download .dmg](https://github.com/theasmat/llm-quota/releases/download/0.1.0/Llm%20Quota_4.2.2_x64.dmg) | [⬇️ Download .exe](https://github.com/theasmat/llm-quota/releases/download/0.1.0/Llm%20Quota_4.2.2_x64-setup.exe) | [⬇️ Download .AppImage](https://github.com/theasmat/llm-quota/releases/download/0.1.0/llm-quota_4.2.2_amd64.AppImage) |
| **arm64 (Apple Silicon/ARM)** | [⬇️ Download .dmg](https://github.com/theasmat/llm-quota/releases/download/0.1.0/Llm%20Quota_4.2.2_aarch64.dmg) | [⬇️ Download .exe](https://github.com/theasmat/llm-quota/releases/download/0.1.0/Llm%20Quota_4.2.2_arm64-setup.exe) | [⬇️ Download .AppImage](https://github.com/theasmat/llm-quota/releases/download/0.1.0/llm-quota_4.2.2_aarch64.AppImage) |
| **Universal (All)** | [⬇️ Download .dmg](https://github.com/theasmat/llm-quota/releases/download/0.1.0/Llm%20Quota_4.2.2_universal.dmg) | - | - |
<!-- DOWNLOAD_TABLE_END -->

### Alternative: macOS (Homebrew)
The easiest way to install on macOS is using our Homebrew tap:
```bash
brew install --no-quarantine theasmat/llm-quota/llm-quota
```

### Alternative: Automated Install Script (macOS & Linux)
```bash
curl -fsSL https://raw.githubusercontent.com/theasmat/llm-quota/master/install.sh | bash
```

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

<br/>

## 📷 Screenshots

> **Note**: These screenshots demonstrate the ultra-compact UI in dark and light themes.
> 
> <p align="center">
  <img src="assets/screenshots/gemini.png" width="45%" alt="Dashboard Dark Theme" />
  &nbsp;&nbsp;&nbsp;
  <img src="assets/screenshots/home.png" width="45%" alt="Dashboard Light Theme" />
</p>

<br/>

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to check out the [issues page](https://github.com/theasmat/llm-quota/issues).

1. **Fork** the project
2. **Create** your feature branch (`git checkout -b feature/AmazingFeature`)
3. **Commit** your changes (`git commit -m 'Add some AmazingFeature'`)
4. **Push** to the branch (`git push origin feature/AmazingFeature`)
5. **Open** a Pull Request

<br/>

## 💬 Feedback & Support

If you have any feedback, please reach out by opening an issue or starting a discussion. If you like the project, please give it a ⭐️!

<br/>

## 📄 License

This project is licensed under the [MIT License](LICENSE).

<div align="center">
  <sub>Built with ❤️ by Asmat</sub>
</div>
