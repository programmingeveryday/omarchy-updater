use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageUpdate {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub source: String, // "Arch", "AUR", "Mise", "Firmware"
}

#[derive(Debug, Clone, Default)]
pub struct UpdateReport {
    pub arch_updates: Vec<PackageUpdate>,
    pub aur_updates: Vec<PackageUpdate>,
    pub omarchy_update_needed: bool,
    pub firmware_updates: Vec<String>,
    pub mise_updates: Vec<String>,
}

impl UpdateReport {
    pub fn total_updates(&self) -> usize {
        self.arch_updates.len()
            + self.aur_updates.len()
            + (if self.omarchy_update_needed { 1 } else { 0 })
            + self.firmware_updates.len()
            + self.mise_updates.len()
    }

    #[allow(dead_code)]
    pub fn is_all_up_to_date(&self) -> bool {
        self.total_updates() == 0
    }
}

pub fn check_arch_updates() -> Vec<PackageUpdate> {
    let mut updates = Vec::new();
    if let Ok(output) = Command::new("checkupdates").output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                // Format: pkgname current_ver -> new_ver
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 && parts[2] == "->" {
                    updates.push(PackageUpdate {
                        name: parts[0].to_string(),
                        current_version: parts[1].to_string(),
                        new_version: parts[3].to_string(),
                        source: "Arch".to_string(),
                    });
                }
            }
        }
    }
    updates
}

pub fn check_aur_updates() -> Vec<PackageUpdate> {
    let mut updates = Vec::new();
    if let Ok(output) = Command::new("yay").args(["-Qua"]).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 && parts[2] == "->" {
                    updates.push(PackageUpdate {
                        name: parts[0].to_string(),
                        current_version: parts[1].to_string(),
                        new_version: parts[3].to_string(),
                        source: "AUR".to_string(),
                    });
                }
            }
        }
    }
    updates
}

pub fn check_omarchy_update() -> bool {
    if let Ok(output) = Command::new("omarchy").args(["update", "available"]).output() {
        output.status.success()
    } else {
        false
    }
}

pub fn check_firmware_updates() -> Vec<String> {
    let mut updates = Vec::new();
    // Use --json which returns {"Devices": []} when up to date, and avoids interactive prompts
    if let Ok(output) = Command::new("fwupdmgr").args(["get-updates", "--json", "--no-history"]).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if let Some(devices) = parsed.get("Devices").and_then(|d| d.as_array()) {
                    for dev in devices {
                        if let Some(name) = dev.get("Name").and_then(|n| n.as_str()) {
                            updates.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    updates
}

pub fn check_mise_updates() -> Vec<String> {
    let mut updates = Vec::new();
    if let Ok(output) = Command::new("mise").arg("outdated").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("mise") {
                updates.push(trimmed.to_string());
            }
        }
    }
    updates
}

pub fn scan_system_updates() -> UpdateReport {
    UpdateReport {
        arch_updates: check_arch_updates(),
        aur_updates: check_aur_updates(),
        omarchy_update_needed: check_omarchy_update(),
        firmware_updates: check_firmware_updates(),
        mise_updates: check_mise_updates(),
    }
}
