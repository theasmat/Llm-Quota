---
name: install-scripts
description: >
  Guide for working with and extending the Llm Quota install scripts.
  Covers the mac.sh and linux.sh scripts, the CI version token pattern,
  local testing with DRY_RUN, and how to add a new platform or package format.
  Trigger: "install script", "mac.sh", "linux.sh", "installer", "add platform",
  "package format", "deb", "rpm", "appimage", "dmg", "dry run", "test installer".
---

# Install Scripts Guide

Llm Quota uses dedicated Bash scripts for one-line installation on macOS and Linux.

---

## Script Locations

| Script | Platform | URL |
|---|---|---|
| `install/mac.sh` | macOS (arm64 + x64) | `curl -fsSL .../install/mac.sh \| bash` |
| `install/linux.sh` | Linux (deb/rpm/AppImage) | `curl -fsSL .../install/linux.sh \| bash` |

Both scripts are designed for piping via `curl | bash` — they avoid interactive TTY features that break when stdin is a pipe.

---

## Architecture of Each Script

```
1. detect_platform()       — Checks uname -s / uname -m
2. detect_linux_distro()   — (Linux only) Checks for apt/dnf/yum
3. get_version()           — GitHub API → redirect fallback → hardcoded fallback
4. build_download_url()    — Interactive menu (if TTY) → constructs GitHub release URL
5. download_installer()    — curl download to /tmp
6. install_macos()         — hdiutil mount → cp to /Applications → xattr quarantine fix
   install_linux()         — dpkg/dnf/yum install or AppImage copy to ~/.local/bin
7. cleanup()               — rm -rf temp dir (trap EXIT)
```

---

## Key Design Patterns

### TTY Detection
All interactive prompts are gated behind:
```bash
if [ -c /dev/tty ]; then
    # Show interactive menu
fi
```
This is critical — when the script is piped via `curl | bash`, there is no TTY and the menu is skipped silently, using the default (detected/recommended) option.

### The `# CI_UPDATED_VERSION` Token
The hardcoded fallback version uses a special comment marker:
```bash
RELEASE_VERSION="0.1.1" # CI_UPDATED_VERSION
```
The CI automatically `sed`-replaces this on every release. **Never remove this comment.**

### Download URL Pattern
GitHub Release assets follow this naming:
- macOS: `Llm.Quota_{version}_{arch}.dmg` where arch = `aarch64` or `x64`
- Linux deb: `Llm.Quota_{version}_{arch}.deb` where arch = `amd64` or `arm64`
- Linux rpm: `Llm.Quota-{version}-1.{arch}.rpm` where arch = `x86_64` or `aarch64`
- Linux AppImage: `Llm.Quota_{version}_{arch}.AppImage` where arch = `amd64` or `aarch64`

---

## Testing Locally

### Dry run (no install, just print commands)
```bash
DRY_RUN=1 bash install/mac.sh
DRY_RUN=1 bash install/linux.sh
```

### Install a specific version
```bash
VERSION=0.1.0 bash install/mac.sh
```

### Validate syntax
```bash
bash -n install/mac.sh
bash -n install/linux.sh
```

### Simulate piped curl (no TTY)
```bash
cat install/mac.sh | DRY_RUN=1 bash
```

---

## How to Add a New Package Format

Example: adding `.pkg` support for macOS.

### Step 1 — Add detection in `detect_platform()` or as a new prompt

In `install/mac.sh`, inside `build_download_url()`:
```bash
echo -e "   [3] 📦 PKG Installer"
```

### Step 2 — Handle the new choice
```bash
elif [[ "$user_arch" == "3" ]]; then
    macos_format="pkg"
fi
```

### Step 3 — Build the URL
```bash
DOWNLOAD_URL="${base_url}/Llm.Quota_${RELEASE_VERSION}.pkg"
FILENAME="Llm.Quota_${RELEASE_VERSION}.pkg"
```

### Step 4 — Add install logic in `install_macos()`
```bash
*.pkg)
    run sudo installer -pkg "$DOWNLOAD_PATH" -target /
    ;;
```

### Step 5 — Ensure the CI builds and uploads the `.pkg` artifact

In `.github/workflows/release.yml`, ensure the `build-macos-*` job produces and uploads the `.pkg` file.

---

## How to Add a New Linux Distro / Package Manager

In `install/linux.sh`, update `detect_linux_distro()`:
```bash
elif command -v zypper &>/dev/null; then
    PKG_MANAGER="zypper"
    PKG_EXT="rpm"
```

And `install_linux()`:
```bash
zypper)
    run sudo zypper install -y "$DOWNLOAD_PATH"
    ;;
```

---

## Checklist

- [ ] All interactive prompts gated with `if [ -c /dev/tty ]`
- [ ] Default behavior (no TTY) uses the auto-detected/recommended option
- [ ] `# CI_UPDATED_VERSION` comment preserved on the fallback version line
- [ ] New URL patterns match actual GitHub Release asset names exactly
- [ ] Script passes `bash -n` syntax check
- [ ] Tested with `DRY_RUN=1` before testing real install
- [ ] Tested via pipe simulation (`cat install/mac.sh | DRY_RUN=1 bash`)
