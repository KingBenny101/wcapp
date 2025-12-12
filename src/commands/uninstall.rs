use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::config;
use self_replace;

pub fn execute() -> Result<()> {
    println!("wcapp Uninstaller");
    println!();

    let current_exe = env::current_exe().context("Failed to get current executable path")?;
    println!("Current installation: {}", current_exe.display());
    println!();

    println!("What would you like to remove?");
    println!("1. Just the wcapp binary");
    println!("2. Binary + configuration");
    println!("3. Binary + configuration + wallpapers");
    println!("4. Cancel");
    println!();

    print!("Enter choice (1-4): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    let mut remove_binary_requested = false;

    match choice {
        "1" => {
            remove_binary(&current_exe)?;
            remove_binary_requested = true;
        }
        "2" => {
            remove_config()?;
            remove_binary(&current_exe)?;
            remove_binary_requested = true;
        }
        "3" => {
            remove_config()?;
            remove_wallpapers()?;
            remove_binary(&current_exe)?;
            remove_binary_requested = true;
        }
        "4" => {
            println!("Uninstall cancelled");
            return Ok(());
        }
        _ => {
            println!("Invalid choice");
            return Ok(());
        }
    }

    println!();
    println!("Uninstall complete!");
    println!();
    #[cfg(target_os = "windows")]
    {
        println!("Note: You may need to manually remove the installation directory from PATH");
        println!("      Go to System Properties > Environment Variables > Path");
    }
    #[cfg(target_os = "macos")]
    {
        println!("Note: You may need to manually remove the installation directory from PATH");
        println!("      Edit ~/.zshrc or ~/.bash_profile and remove the directory from PATH");
    }
    #[cfg(target_os = "linux")]
    {
        println!("Note: You may need to manually remove the installation directory from PATH");
        println!("      Edit ~/.bashrc or ~/.profile and remove the directory from PATH");
    }

    // If binary removal was requested, delete it now (this will exit the process)
    if remove_binary_requested {
        println!("Removing binary...");
        self_replace::self_delete().context("Failed to remove binary")?;
        // This line will never be reached - self_delete() exits the process
    }

    Ok(())
}

fn remove_binary(exe_path: &PathBuf) -> Result<()> {
    println!();
    print!("Remove wcapp binary? This action cannot be undone. (y/N): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" {
        println!("Skipped binary removal");
        return Ok(());
    }

    // Check permissions before attempting self-deletion
    #[cfg(not(target_os = "windows"))]
    {
        // Check if we can write to the binary's directory
        if let Some(parent) = exe_path.parent() {
            if let Ok(metadata) = fs::metadata(parent) {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                let can_write = permissions.mode() & 0o200 != 0; // Check write permission

                // Check if it's a system directory that typically requires sudo
                let is_system_dir = exe_path.starts_with("/usr")
                    || exe_path.starts_with("/bin")
                    || exe_path.starts_with("/sbin")
                    || exe_path.starts_with("/opt") && !exe_path.starts_with("/opt/homebrew");

                if !can_write || is_system_dir {
                    println!();
                    println!(
                        "✗ Insufficient permissions to remove {}",
                        exe_path.display()
                    );
                    println!("Please run with sudo:");
                    println!("  sudo {} uninstall", exe_path.display());
                    return Ok(());
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Check if we can write to the directory
        if let Some(parent) = exe_path.parent() {
            if let Ok(metadata) = fs::metadata(parent) {
                use std::os::windows::fs::MetadataExt;
                let attributes = metadata.file_attributes();
                let is_readonly = attributes & 0x1 != 0; // FILE_ATTRIBUTE_READONLY

                if is_readonly {
                    println!("✗ Cannot remove binary - directory is read-only");
                    return Ok(());
                }
            }
        }
    }

    println!("✓ Binary will be removed when uninstall completes");
    Ok(())
}

fn remove_config() -> Result<()> {
    if let Ok(config_path) = config::get_config_path() {
        if config_path.exists() {
            fs::remove_file(&config_path).context("Failed to remove config file")?;
            println!("✓ Configuration removed");

            if let Some(parent) = config_path.parent() {
                if parent.read_dir()?.next().is_none() {
                    if let Err(e) = fs::remove_dir(parent) {
                        println!("Note: Could not remove empty config directory: {}", e);
                    }
                }
            }
        } else {
            println!("✓ No configuration found");
        }
    }
    Ok(())
}

fn remove_wallpapers() -> Result<()> {
    if let Ok(wallpaper_dir) = config::get_wallpaper_dir() {
        if wallpaper_dir.exists() {
            print!("Remove {} wallpapers? (y/N): ", wallpaper_dir.display());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                fs::remove_dir_all(&wallpaper_dir).context("Failed to remove wallpapers")?;
                println!("✓ Wallpapers removed");
            } else {
                println!("✓ Wallpapers kept");
            }
        } else {
            println!("✓ No wallpapers found");
        }
    }
    Ok(())
}
