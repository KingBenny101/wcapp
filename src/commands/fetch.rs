use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

use crate::config::{self, Config, WALLPAPER_REPO};

/// Fetch wallpapers from a git repository and move images to destination
pub fn execute(destination: Option<PathBuf>) -> Result<()> {
    println!("Fetching wallpapers from: {}", WALLPAPER_REPO);

    let dest_dir = destination.unwrap_or_else(|| {
        config::get_wallpaper_dir().expect("Could not determine wallpaper directory")
    });

    println!("Destination: {}", dest_dir.display());

    let temp_dir = std::env::temp_dir().join("wcapp_temp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    println!("Cloning repository...");
    let status = Command::new("git")
        .args(["clone", "--depth", "1", WALLPAPER_REPO])
        .arg(&temp_dir)
        .status()
        .context("Failed to execute git. Make sure git is installed and in PATH")?;

    if !status.success() {
        anyhow::bail!("Git clone failed");
    }

    println!("Copying images with folder structure...");
    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut copied_count = 0;

    let classified_dir = temp_dir.join("classified");

    if !classified_dir.exists() {
        anyhow::bail!("classified/ folder not found in repository");
    }

    for entry in WalkDir::new(&classified_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if image_extensions.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str()) {
                    if let Ok(relative_path) = path.strip_prefix(&classified_dir) {
                        let dest_path = dest_dir.join(relative_path);

                        if let Some(parent) = dest_path.parent() {
                            fs::create_dir_all(parent)
                                .context("Failed to create category directory")?;
                        }

                        fs::copy(path, &dest_path)
                            .context(format!("Failed to copy {}", relative_path.display()))?;
                        copied_count += 1;
                    }
                }
            }
        }
    }

    fs::remove_dir_all(&temp_dir)?;

    println!(
        "Successfully copied {} wallpapers to {}",
        copied_count,
        dest_dir.display()
    );

    let config = Config {
        wallpaper_dir: dest_dir.clone(),
    };
    config::save_config(&config)?;

    println!("Wallpaper directory saved to config");

    Ok(())
}
