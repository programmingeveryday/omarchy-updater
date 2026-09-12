# Omarchy System Updates (`omarchy-updater`)

A beautiful, native GTK4 & Libadwaita graphical application designed specifically for [Omarchy](https://omarchy.org/) Linux to inspect, report, and manage system updates across all repositories, firmware, and toolchains.

---

## ✨ Features

- **🎨 Native Omarchy Aesthetic:**
  - Dynamically parses the active theme configuration (`~/.local/state/omarchy/current/theme/colors.toml`).
  - Seamlessly adapts window backgrounds, cards, typography, and status badges to your active desktop theme (Light & Dark mode support).
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
  - Pre-configured to launch centered as a floating window rather than tiling, fitting right into your workflow.
- **🔄 Live Refresh:**
  - Dedicated refresh button in the header bar to re-scan all repositories on demand.

---

## 🚀 Installation & Setup

### Pre-built Binary (GitHub Releases)
Download the latest `omarchy-updater-linux-x86_64.tar.gz` from the [Releases](https://github.com/programmingeveryday/omarchy-updater/releases) page:

```bash
tar -xzvf omarchy-updater-linux-x86_64.tar.gz
install -Dm755 bin/omarchy-updater ~/.local/bin/omarchy-updater
install -Dm644 share/applications/omarchy-updater.desktop ~/.local/share/applications/omarchy-updater.desktop
```

### Build from Source

Ensure you have Rust and the GTK4 / Libadwaita libraries installed:

```bash
# Clone the repository
git clone https://github.com/programmingeveryday/omarchy-updater.git
cd omarchy-updater

# Build optimized release binary
cargo build --release

# Install locally
install -Dm755 target/release/omarchy-updater ~/.local/bin/omarchy-updater
install -Dm644 extra/omarchy-updater.desktop ~/.local/share/applications/omarchy-updater.desktop
```

---

## 🪟 Hyprland Configuration

To have `omarchy-updater` open centered and floating automatically under Hyprland, add the following window rule to `~/.config/hypr/hyprland.lua`:

```lua
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
