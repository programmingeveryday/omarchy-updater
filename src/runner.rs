use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

pub enum UpdateEvent {
    Log(String),
    Finished(bool), // true = success, false = failure
}

pub fn run_embedded_update(tx: Sender<UpdateEvent>) {
    std::thread::spawn(move || {
        let _ = tx.send(UpdateEvent::Log("Requesting authentication & starting Omarchy update...\n".to_string()));

        // Run omarchy update using pkexec so Omarchy's native Polkit agent
        // presents the GUI fingerprint / password prompt.
        let mut child = match Command::new("pkexec")
            .args(["omarchy", "update", "-y"])
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

        // Spawn thread for stdout
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

        // Spawn thread for stderr
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
                    let _ = tx.send(UpdateEvent::Log(format!("\n⚠ Update exited with code: {:?}\n", status.code())));
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
