use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

pub enum UpdateEvent {
    Log(String),
    Finished(bool),
}

pub fn run_embedded_update(tx: Sender<UpdateEvent>) {
    std::thread::spawn(move || {
        let _ = tx.send(UpdateEvent::Log("Starting Omarchy update...\n".to_string()));

        // Run as the current user, passing sudo inside the command.
        // We use pkexec to cache the sudo credentials upfront if needed, OR
        // we run bash -c with environment configured.
        // Because omarchy update executes migrations, yay (AUR), mise, and user hooks,
        // running the root updater under pkexec as root directly broke $HOME and $OMARCHY_PATH.
        // Instead, we invoke:
        // pkexec --user root env OMARCHY_PATH=/usr/share/omarchy /usr/share/omarchy/bin/omarchy-update -y
        let script = r#"
            export OMARCHY_PATH=/usr/share/omarchy
            export OMARCHY_UPDATE_UNATTENDED=1
            pkexec env OMARCHY_PATH=/usr/share/omarchy /usr/share/omarchy/bin/omarchy-update -y
        "#;

        let mut child = match Command::new("bash")
            .args(["-c", script])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                let _ = tx.send(UpdateEvent::Log(format!("Error initiating update: {}\n", e)));
                let _ = tx.send(UpdateEvent::Finished(false));
                return;
            }
        };

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let tx_stdout = tx.clone();
        let handle_stdout = std::thread::spawn(move || {
            if let Some(out) = stdout {
                let reader = BufReader::new(out);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        let _ = tx_stdout.send(UpdateEvent::Log(format!("{}\n", l)));
                    }
                }
            }
        });

        let tx_stderr = tx.clone();
        let handle_stderr = std::thread::spawn(move || {
            if let Some(err) = stderr {
                let reader = BufReader::new(err);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        let _ = tx_stderr.send(UpdateEvent::Log(format!("{}\n", l)));
                    }
                }
            }
        });

        let _ = handle_stdout.join();
        let _ = handle_stderr.join();

        match child.wait() {
            Ok(status) => {
                let success = status.success();
                if success {
                    let _ = tx.send(UpdateEvent::Log("\n✓ System update completed successfully!\n".to_string()));
                } else {
                    let _ = tx.send(UpdateEvent::Log(format!("\n⚠ Update process finished (code: {:?})\n", status.code())));
                }
                let _ = tx.send(UpdateEvent::Finished(success));
            }
            Err(e) => {
                let _ = tx.send(UpdateEvent::Log(format!("\nFailed to wait on update process: {}\n", e)));
                let _ = tx.send(UpdateEvent::Finished(false));
            }
        }
    });
}
