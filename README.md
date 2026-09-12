# Omarchy System Updates (`omarchy-updater`)

A modern, native GTK4 & Libadwaita graphical application designed for Omarchy Linux to check, report, and manage system updates across all repositories and subsystems.

## Features

- **Omarchy Design Language:** Dynamically parses the active theme (`~/.local/state/omarchy/current/theme/colors.toml`) to style background, card surfaces, and status badges.
- **Repository Inspection:**
  - **Arch Linux Repositories:** Scans official updates using `checkupdates`.
  - **Arch User Repository (AUR):** Scans AUR packages using `yay -Qua`.
  - **Omarchy Core:** Checks for OS migrations and core updates via `omarchy update available`.
  - **Hardware Firmware:** Inspects LVFS device updates via `fwupdmgr`.
  - **Developer Toolchains:** Scans pinned runtimes using `mise outdated`.
- **Interactive Update Runner:** One-click **Update Now** button that launches Omarchy's complete update workflow in a terminal (Ghostty/Foot) for secure privilege escalation.
- **Floating Window:** Pre-configured to float centered on the screen under Hyprland.

## Building & Installation

Ensure you have Rust, Cargo, and the GTK4 / Libadwaita development libraries installed:

```bash
# Clone the repository
git clone https://github.com/<username>/omarchy-updater.git
cd omarchy-updater

# Build optimized binary
cargo build --release

# Install binary
install -Dm755 target/release/omarchy-updater ~/.local/bin/omarchy-updater

# Install desktop entry
install -Dm644 extra/omarchy-updater.desktop ~/.local/share/applications/omarchy-updater.desktop
```

## License
MIT
