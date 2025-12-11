use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

/// Update wcapp to the latest version
pub fn execute() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: {}", current_version);
    println!("Checking for updates...");
    println!();

    let client = reqwest::blocking::Client::builder()
        .user_agent("wcapp")
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get("https://api.github.com/repos/KingBenny101/wcapp/releases/latest")
        .send()
        .context("Failed to fetch latest release information")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to check for updates: HTTP {}", response.status());
    }

    let release_info: serde_json::Value = response
        .json()
        .context("Failed to parse release information")?;

    let latest_version = release_info["tag_name"]
        .as_str()
        .context("Could not find latest version")?;

    let latest_version = latest_version.trim_start_matches('v');

    if latest_version == current_version {
        println!(
            "You are already running the latest version ({}).",
            current_version
        );
        return Ok(());
    }

    if is_newer_version(latest_version, current_version) {
        println!("New version available: {}", latest_version);
        println!("Updating wcapp...");
        println!();
    } else {
        println!("You are running a newer version than the latest release.");
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        let script_url = "https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.ps1";
        let current_pid = std::process::id();

        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("wcapp_update.bat");

        let script_content = format!(
            r#"@echo off
title wcapp Updater
echo Waiting for wcapp to close...
:wait
tasklist /FI "PID eq {}" 2>NUL | find "{}" >NUL
if %ERRORLEVEL% EQU 0 (
    timeout /t 1 /nobreak >NUL
    goto wait
)
echo Downloading update...
powershell -ExecutionPolicy Bypass -Command "irm {} | iex"
pause
del "%~f0"
"#,
            current_pid, current_pid, script_url
        );

        fs::write(&script_path, script_content).context("Failed to create update script")?;

        // Launch the batch script elevated
        Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Start-Process -FilePath '{}' -Verb RunAs",
                    script_path.to_str().unwrap()
                ),
            ])
            .spawn()
            .context("Failed to launch update script")?;

        println!("Update will continue in an elevated window after this process exits.");
        std::process::exit(0);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let script_url = "https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.sh";
        let current_pid = std::process::id();

        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("wcapp_update.sh");

        let script_content = format!(
            r#"#!/bin/sh
while kill -0 {} 2>/dev/null; do
    sleep 1
done
curl -fsSL {} | sh
rm -f "$0"
"#,
            current_pid, script_url
        );

        fs::write(&script_path, script_content).context("Failed to create update script")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)?;
        }

        use std::os::unix::process::CommandExt;
        println!("Running installer in this terminal after wcapp exits...");
        // Replace current process with the update script (so prompts work)
        Command::new("sh").arg(&script_path).exec();
        // If exec fails:
        anyhow::bail!("Failed to launch update script");
    }
}

/// Compare two semantic versions (simple implementation)
fn is_newer_version(latest: &str, current: &str) -> bool {
    let parse_version =
        |v: &str| -> Vec<u32> { v.split('.').filter_map(|s| s.parse::<u32>().ok()).collect() };

    let latest_parts = parse_version(latest);
    let current_parts = parse_version(current);

    for i in 0..latest_parts.len().max(current_parts.len()) {
        let l = latest_parts.get(i).unwrap_or(&0);
        let c = current_parts.get(i).unwrap_or(&0);

        if l > c {
            return true;
        } else if l < c {
            return false;
        }
    }

    false
}
