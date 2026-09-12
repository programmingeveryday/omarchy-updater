use std::process::Command;

pub fn launch_update_in_terminal() -> Result<(), String> {
    // Find terminal
    let term = if std::path::Path::new("/usr/bin/ghostty").exists() {
        "ghostty"
    } else if std::path::Path::new("/usr/bin/foot").exists() {
        "foot"
    } else {
        "xterm"
    };

    let script = r#"
        echo -e "\033[1;34m=== Omarchy System & App Updater ===\033[0m\n"
        omarchy update
        echo -e "\n\033[1;32mUpdate process finished.\033[0m Press Enter to close."
        read -r
    "#;

    let child = Command::new(term)
        .arg("-e")
        .args(["bash", "-c", script])
        .spawn();

    match child {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to spawn terminal updater: {}", e)),
    }
}
