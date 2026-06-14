#!/usr/bin/env bash
# Llm Quota Install Script (Linux)
# Usage: curl -fsSL https://raw.githubusercontent.com/theasmat/llm-quota/master/install/linux.sh | bash
#
# Environment variables:
#   VERSION     - Install specific version (e.g., "0.1.1"), default: latest
#   DRY_RUN     - Set to "1" to print commands without executing

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO="theasmat/llm-quota"
APP_NAME="Llm Quota"
APP_ID="com.theasmat.llm-quota"
GITHUB_API="https://api.github.com/repos/${REPO}/releases"

# Helper functions
info() { echo -e "${BLUE}ℹ️  [INFO]${NC} $1"; }
success() { echo -e "${GREEN}✅ [OK]${NC} $1"; }
warn() { echo -e "${YELLOW}⚠️  [WARN]${NC} $1"; }
error() { echo -e "${RED}❌ [ERROR]${NC} $1" >&2; exit 1; }
prompt() { echo -e -n "${YELLOW}👉 [PROMPT]${NC} $1"; }

run() {
    if [[ "${DRY_RUN:-0}" == "1" ]]; then
        echo -e "${YELLOW}[DRY-RUN]${NC} $*"
    else
        "$@"
    fi
}

# Show help
show_help() {
    cat << EOF
${APP_NAME} Linux Install Script

Usage:
    curl -fsSL https://raw.githubusercontent.com/${REPO}/master/install/linux.sh | bash

    # Install specific version
    curl -fsSL https://raw.githubusercontent.com/${REPO}/master/install/linux.sh | VERSION=0.1.1 bash

Options:
    --help      Show this help message
    --version   Show script version

Environment Variables:
    VERSION     Install specific version (default: latest)
    DRY_RUN     Set to "1" to preview commands without executing

Supported Platforms:
    - Linux x86_64:  .deb (Debian/Ubuntu), .rpm (Fedora/RHEL), .AppImage (Universal)
    - Linux aarch64: .deb (Debian/Ubuntu), .rpm (Fedora/RHEL), .AppImage (Universal)

EOF
    exit 0
}

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    if [[ "$OS" != "Linux" ]]; then
        error "This script is for Linux. Use install/mac.sh for macOS or install.ps1 for Windows."
    fi

    PLATFORM="linux"
    PLATFORM_ICON="🐧 linux"

    case "$ARCH" in
        x86_64|amd64)   ARCH_LABEL="x86_64"; DEB_ARCH="amd64"; RPM_ARCH="x86_64" ;;
        aarch64|arm64)  ARCH_LABEL="aarch64"; DEB_ARCH="arm64"; RPM_ARCH="aarch64" ;;
        *)              error "Unsupported architecture: $ARCH" ;;
    esac

    info "Detected: $PLATFORM_ICON ($ARCH_LABEL)"
}

# Detect Linux package manager
detect_linux_distro() {
    if command -v apt-get &>/dev/null; then
        PKG_MANAGER="apt"
        PKG_EXT="deb"
    elif command -v dnf &>/dev/null; then
        PKG_MANAGER="dnf"
        PKG_EXT="rpm"
    elif command -v yum &>/dev/null; then
        PKG_MANAGER="yum"
        PKG_EXT="rpm"
    else
        PKG_MANAGER="appimage"
        PKG_EXT="AppImage"
        warn "No supported package manager found, using AppImage"
    fi

    info "Package manager: $PKG_MANAGER ($PKG_EXT)"
}

# Get latest or specific version
get_version() {
    if [[ -n "${VERSION:-}" ]]; then
        RELEASE_VERSION="$VERSION"
        info "Using specified version: v$RELEASE_VERSION"
        return
    fi

    info "Fetching latest version..."

    # Method 1: Try GitHub API
    local response
    if response=$(curl -fsSL -H "User-Agent: Antigravity-Installer" "${GITHUB_API}/latest" 2>/dev/null); then
        RELEASE_VERSION=$(echo "$response" | awk -F'"tag_name": *"' '{print $2}' | awk -F'"' '{print $1}' | sed 's/^v//')
        if [[ -n "$RELEASE_VERSION" ]]; then
            info "Latest version: v$RELEASE_VERSION"
            return
        fi
    fi

    # Method 2: Fallback - parse from redirect URL (no rate limit)
    info "API rate limited, using fallback method..."
    local redirect_url
    redirect_url=$(curl -fsSI "https://github.com/${REPO}/releases/latest" 2>/dev/null | grep -i "^location:" | tr -d '\r' | awk '{print $2}')

    if [[ -n "$redirect_url" && "$redirect_url" == *"/tag/"* ]]; then
        RELEASE_VERSION=$(echo "$redirect_url" | sed -E 's|.*/tag/v?||')
    fi

    # Method 3: Hardcoded fallback updated by CI
    if [[ -z "${RELEASE_VERSION:-}" ]]; then
        RELEASE_VERSION="0.1.1" # CI_UPDATED_VERSION
        warn "Failed to fetch latest version from network. Falling back to hardcoded v$RELEASE_VERSION"
    fi

    if [ -c /dev/tty ]; then
        prompt "Latest version is v$RELEASE_VERSION. Enter version to install or press Enter for latest [v$RELEASE_VERSION]: "
        if read -r user_ver < /dev/tty; then
            user_ver=$(echo "$user_ver" | tr -d '[:space:]')
            if [[ -n "$user_ver" && "$user_ver" != "y" && "$user_ver" != "Y" ]]; then
                RELEASE_VERSION="${user_ver#v}"
            fi
        fi
    fi

    info "Selected version: v$RELEASE_VERSION"
}

# Build download URL based on platform and package manager
build_download_url() {
    local base_url="https://github.com/${REPO}/releases/download/v${RELEASE_VERSION}"

    case "$PKG_EXT" in
        deb)
            DOWNLOAD_URL="${base_url}/Llm.Quota_${RELEASE_VERSION}_${DEB_ARCH}.deb"
            FILENAME="Llm.Quota_${RELEASE_VERSION}_${DEB_ARCH}.deb"
            ;;
        rpm)
            DOWNLOAD_URL="${base_url}/Llm.Quota-${RELEASE_VERSION}-1.${RPM_ARCH}.rpm"
            FILENAME="Llm.Quota-${RELEASE_VERSION}-1.${RPM_ARCH}.rpm"
            ;;
        AppImage)
            local appimage_arch
            if [[ "$ARCH_LABEL" == "x86_64" ]]; then
                appimage_arch="amd64"
            else
                appimage_arch="aarch64"
            fi
            DOWNLOAD_URL="${base_url}/Llm.Quota_${RELEASE_VERSION}_${appimage_arch}.AppImage"
            FILENAME="Llm.Quota_${RELEASE_VERSION}_${appimage_arch}.AppImage"
            ;;
    esac

    info "Download URL: $DOWNLOAD_URL"
}

# Download installer
download_installer() {
    TEMP_DIR=$(mktemp -d)
    DOWNLOAD_PATH="${TEMP_DIR}/${FILENAME}"

    info "Downloading ${APP_NAME} v${RELEASE_VERSION}..."
    run curl -fSL --progress-bar -o "$DOWNLOAD_PATH" "$DOWNLOAD_URL"

    if [[ "${DRY_RUN:-0}" != "1" ]] && [[ ! -f "$DOWNLOAD_PATH" ]]; then
        error "Download failed. Check your network or try a different version."
    fi

    success "Downloaded to $DOWNLOAD_PATH"
}

# Install on Linux
install_linux() {
    info "Installing ${APP_NAME}..."

    case "$PKG_MANAGER" in
        apt)
            run sudo dpkg -i "$DOWNLOAD_PATH"
            run sudo apt-get install -f -y  # Fix dependencies if needed
            ;;
        dnf)
            run sudo dnf install -y "$DOWNLOAD_PATH"
            ;;
        yum)
            run sudo yum install -y "$DOWNLOAD_PATH"
            ;;
        appimage)
            local install_dir="${HOME}/.local/bin"
            run mkdir -p "$install_dir"
            run chmod +x "$DOWNLOAD_PATH"
            run cp "$DOWNLOAD_PATH" "${install_dir}/llm-quota"

            if [[ ":$PATH:" != *":${install_dir}:"* ]]; then
                warn "Add ${install_dir} to your PATH to run llm-quota from anywhere"

                local shell_name rc_file export_line
                shell_name="$(basename "${SHELL:-/bin/bash}")"
                case "$shell_name" in
                    zsh)  rc_file="$HOME/.zshrc" ;;
                    fish) rc_file="$HOME/.config/fish/config.fish" ;;
                    *)    rc_file="$HOME/.bashrc" ;;
                esac

                export_line="export PATH=\"${install_dir}:\$PATH\""
                [[ "$shell_name" == "fish" ]] && export_line="fish_add_path ${install_dir}"

                if [[ -f "$rc_file" ]] && grep -qF "$install_dir" "$rc_file" 2>/dev/null; then
                    info "PATH entry already in $rc_file"
                else
                    run echo "$export_line" >> "$rc_file"
                    info "Added ${install_dir} to PATH in $rc_file"
                    warn "Run: source $rc_file  (or restart terminal)"
                fi
            fi
            ;;
    esac

    success "${APP_NAME} installed successfully!"
}

# Cleanup
cleanup() {
    if [[ -n "${TEMP_DIR:-}" ]] && [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
}

# Main
main() {
    for arg in "$@"; do
        case "$arg" in
            --help|-h)    show_help ;;
            --version|-v) echo "linux.sh v1.0.0"; exit 0 ;;
        esac
    done

    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}   🚀 ${APP_NAME} Installer${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""

    trap cleanup EXIT

    detect_platform
    detect_linux_distro
    get_version
    build_download_url
    download_installer
    install_linux

    echo ""
    success "Installation complete!"
    echo ""
    info "Launch '${APP_NAME}' from your application menu or launcher."
    echo ""
}

main "$@"
