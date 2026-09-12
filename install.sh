#!/usr/bin/env bash
#
# install.sh - Installer for Omarchy System Updates (omarchy-updater)
# Usage:
#   curl -sSL https://raw.githubusercontent.com/programmingeveryday/omarchy-updater/main/install.sh | bash
#   or run locally inside the extracted tarball: ./install.sh
#

set -euo pipefail

REPO="programmingeveryday/omarchy-updater"
BIN_NAME="omarchy-updater"
INSTALL_BIN_DIR="$HOME/.local/bin"
INSTALL_APP_DIR="$HOME/.local/share/applications"
HYPR_CONFIG="$HOME/.config/hypr/hyprland.lua"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}=== Installing Omarchy System Updates ===${NC}\n"

# 1. Ensure target directories exist
mkdir -p "$INSTALL_BIN_DIR" "$INSTALL_APP_DIR"

# 2. Check if running from local extracted tarball or downloading from GitHub
if [[ -f "./bin/$BIN_NAME" ]] && [[ -f "./share/applications/$BIN_NAME.desktop" ]]; then
    echo -e "${GREEN}✓ Found local installation files.${NC}"
    install -Dm755 "./bin/$BIN_NAME" "$INSTALL_BIN_DIR/$BIN_NAME"
    install -Dm644 "./share/applications/$BIN_NAME.desktop" "$INSTALL_APP_DIR/$BIN_NAME.desktop"
elif [[ -f "./target/release/$BIN_NAME" ]] && [[ -f "./extra/$BIN_NAME.desktop" ]]; then
    echo -e "${GREEN}✓ Installing from build directory.${NC}"
    install -Dm755 "./target/release/$BIN_NAME" "$INSTALL_BIN_DIR/$BIN_NAME"
    install -Dm644 "./extra/$BIN_NAME.desktop" "$INSTALL_APP_DIR/$BIN_NAME.desktop"
else
    echo -e "Fetching latest release from GitHub (${REPO})..."
    LATEST_TAG=$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | head -n1 | cut -d '"' -f 4 || echo "")

    if [[ -z "$LATEST_TAG" ]]; then
        # Fallback: query releases list if latest release is draft/prerelease
        LATEST_TAG=$(curl -sSL "https://api.github.com/repos/${REPO}/releases" | grep '"tag_name":' | head -n1 | cut -d '"' -f 4 || echo "")
    fi

    if [[ -z "$LATEST_TAG" ]]; then
        echo -e "${RED}Error: Could not retrieve release tags from GitHub.${NC}"
        exit 1
    fi

    echo -e "Downloading version ${GREEN}${LATEST_TAG}${NC}..."
    TAR_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/omarchy-updater-linux-x86_64.tar.gz"
    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT

    curl -sSL "$TAR_URL" -o "$TMP_DIR/omarchy-updater.tar.gz"
    tar -xzf "$TMP_DIR/omarchy-updater.tar.gz" -C "$TMP_DIR"

    install -Dm755 "$TMP_DIR/bin/$BIN_NAME" "$INSTALL_BIN_DIR/$BIN_NAME"
    install -Dm644 "$TMP_DIR/share/applications/$BIN_NAME.desktop" "$INSTALL_APP_DIR/$BIN_NAME.desktop"
fi

# 3. Update desktop database
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$INSTALL_APP_DIR" 2>/dev/null || true
fi

# 4. Check & configure Hyprland window rule
RULE='o.window("org.omarchy.updater", { float = true, center = true, size = { 720, 620 } })'

if [[ -f "$HYPR_CONFIG" ]]; then
    if ! grep -q "org.omarchy.updater" "$HYPR_CONFIG"; then
        echo -e "\nAdding floating & centered window rule to ${HYPR_CONFIG}..."
        echo -e "\n-- Omarchy System Updates floating window\n$RULE" >> "$HYPR_CONFIG"
        if command -v hyprctl >/dev/null 2>&1; then
            hyprctl reload >/dev/null 2>&1 || true
        fi
        echo -e "${GREEN}✓ Hyprland window rule added and reloaded.${NC}"
    else
        echo -e "${GREEN}✓ Hyprland window rule already present.${NC}"
    fi
fi

# 5. Ensure ~/.local/bin is on PATH
if [[ ":$PATH:" != *":$INSTALL_BIN_DIR:"* ]]; then
    echo -e "${YELLOW}Note: $INSTALL_BIN_DIR is not currently in your PATH.${NC}"
    echo -e "Add ${BLUE}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC} to your shell rc file."
fi

echo -e "\n${GREEN}🎉 Omarchy System Updates successfully installed!${NC}"
echo -e "You can launch it from your application menu (Super key) or run: ${BLUE}omarchy-updater${NC}\n"
