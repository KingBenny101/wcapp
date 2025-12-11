use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::config;

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

    match choice {
        "1" => {
            remove_binary(&current_exe)?;
        }
        "2" => {
            remove_config()?;
            remove_binary(&current_exe)?;
        }
        "3" => {
            remove_config()?;
            remove_wallpapers()?;
            remove_binary(&current_exe)?;
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
    println!("Note: You may need to manually remove the installation directory from PATH");

    #[cfg(target_os = "windows")]
    {
        if let Some(parent) = current_exe.parent() {
            println!("Directory: {}", parent.display());

            // Launch batch script to delete the binary after exit
            let batch_script = parent.join("uninstall_wcapp.bat");
            if batch_script.exists() {
                std::process::Command::new("cmd")
                    .args([
                        "/C",
                        "start",
                        "/MIN",
                        "cmd",
                        "/C",
                        batch_script.to_str().unwrap(),
                    ])
                    .spawn()
                    .ok();
            }
        }
    }

    std::process::exit(0);
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

    #[cfg(target_os = "windows")]
    {
        let parent = exe_path
            .parent()
            .context("Failed to get parent directory")?;
        let batch_script = parent.join("uninstall_wcapp.bat");

        let script_content = format!(
            "@echo off\r\n\
             timeout /t 2 /nobreak >nul 2>&1\r\n\
             del /f /q \"{}\" >nul 2>&1\r\n\
             del /f /q \"%~f0\" >nul 2>&1\r\n",
            exe_path.display()
        );

        fs::write(&batch_script, script_content).context("Failed to create uninstall script")?;

        println!("✓ Binary removal scheduled");
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;

        // Check if we can write to the binary's directory
        if let Some(parent) = exe_path.parent() {
            if let Ok(metadata) = fs::metadata(parent) {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                let can_write = permissions.mode() & 0o200 != 0; // Check write permission

                if !can_write || exe_path.starts_with("/usr") {
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

        fs::remove_file(exe_path).context("Failed to remove binary")?;
        println!("✓ Binary removed");
    }

    Ok(())
}

fn remove_config() -> Result<()> {
    if let Ok(config_path) = config::get_config_path() {
        if config_path.exists() {
            fs::remove_file(&config_path).context("Failed to remove config file")?;
            println!("✓ Configuration removed");

            if let Some(parent) = config_path.parent() {
                if parent.read_dir()?.next().is_none() {
                    fs::remove_dir(parent).ok();
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
