# Omarchy System Updates (`omarchy-updater`)

A beautiful, native GTK4 & Libadwaita graphical application designed specifically for [Omarchy](https://omarchy.org/) Linux to inspect, report, and manage system updates across all repositories, firmware, and toolchains.

---

## ✨ Features

- **🎨 Native Omarchy Aesthetic:**
  - Dynamically parses the active theme configuration (`~/.local/state/omarchy/current/theme/colors.toml`).
  - Seamlessly adapts window backgrounds, cards, typography, and status badges to your active desktop theme (supports all light and dark Omarchy themes).
- **🔍 Comprehensive Subsystem Inspection:**
  - **Arch Linux Repositories:** Scans official updates safely using `checkupdates`.
  - **AUR (Arch User Repository):** Scans AUR packages using `yay -Qua`.
  - **Omarchy Core:** Checks for Omarchy system migrations and core releases via `omarchy update available`.
  - **Hardware Firmware:** Inspects device and UEFI firmware status via `fwupdmgr` (LVFS).
  - **Developer Toolchains:** Checks version status for developer tools managed by `mise`.
- **⚡ Interactive "Update Now" Runner:**
  - Prominent update action that triggers when any updates are pending.
  - Automatically launches the complete Omarchy update workflow within a native terminal (Ghostty/Foot) for secure privilege escalation (sudo / biometric authentication) and real-time migration logs.
- **🪟 Hyprland Ready:**
  - Automatically opens centered as a floating window rather than tiling.
- **🔄 Live Refresh:**
  - Dedicated refresh button in the header bar to re-scan all repositories on demand.

---

## 🚀 Quick Install (Recommended)

You can install `omarchy-updater` in one command:

```bash
curl -sSL https://raw.githubusercontent.com/programmingeveryday/omarchy-updater/main/install.sh | bash
```

### What this script does automatically:
1. Downloads the latest pre-compiled release binary and desktop icon.
2. Installs the executable to `~/.local/bin/omarchy-updater`.
3. Registers the desktop launcher in `~/.local/share/applications/` so it appears in your Omarchy `Super` menu.
4. Adds the floating and centered window rule to your `~/.config/hypr/hyprland.lua` (if not already present) and reloads Hyprland.

---

## 📦 Manual Installation from Release Archive

1. Download `omarchy-updater-linux-x86_64.tar.gz` from the [Releases](https://github.com/programmingeveryday/omarchy-updater/releases) page.
2. Extract the archive and run the included installer:

```bash
tar -xzf omarchy-updater-linux-x86_64.tar.gz
./install.sh
```

---

## 🔨 Build from Source

If you prefer compiling directly from source:

```bash
# 1. Clone the repository
git clone https://github.com/programmingeveryday/omarchy-updater.git
cd omarchy-updater

# 2. Build the optimized release binary
cargo build --release

# 3. Run the installer
./install.sh
```

---

## 🪟 Hyprland Configuration (Manual)

The installer configures this automatically, but if you want to set it up manually in `~/.config/hypr/hyprland.lua`:

```lua
-- Omarchy System Updates floating window rule
o.window("org.omarchy.updater", { float = true, center = true, size = { 720, 620 } })
```

Then reload Hyprland:
```bash
hyprctl reload
```

---

## 🛠️ Tech Stack

- **Language:** Rust (2024 edition)
- **GUI Toolkit:** GTK4 & Libadwaita
- **Async Runtime:** Tokio & GLib event loop
- **Theming:** Dynamic CSS injection from Omarchy's theme engine

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
