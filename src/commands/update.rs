use anyhow::{Context, Result};
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
        println!("Launching installer...");
        println!("The update will continue after this process exits.");
        println!();

        Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &format!("Start-Sleep -Milliseconds 500; irm {} | iex", script_url),
            ])
            .spawn()
            .context("Failed to execute PowerShell. Make sure PowerShell is available.")?;

        std::process::exit(0);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let script_url = "https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.sh";
        println!("Launching installer...");
        println!("The update will continue after this process exits.");
        println!();

        Command::new("sh")
            .args(["-c", &format!("sleep 0.5; curl -fsSL {} | sh", script_url)])
            .spawn()
            .context("Failed to execute update script. Make sure curl and sh are available.")?;

        std::process::exit(0);
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
