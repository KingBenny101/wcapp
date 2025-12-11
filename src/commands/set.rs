use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::config;

/// Set a specific wallpaper by name
pub fn execute(name: Option<String>, random: bool) -> Result<()> {
    if random {
        set_random_wallpaper()
    } else if let Some(name) = name {
        set_wallpaper(&name)
    } else {
        anyhow::bail!("Please provide a wallpaper name with --name or use --random flag");
    }
}

/// Set a specific wallpaper by name
fn set_wallpaper(name: &str) -> Result<()> {
    let wallpaper_dir = config::get_wallpaper_dir()?;
    let wallpaper_path = wallpaper_dir.join(name);

    if !wallpaper_path.exists() {
        anyhow::bail!(
            "Wallpaper '{}' not found in {}",
            name,
            wallpaper_dir.display()
        );
    }

    set_wallpaper_internal(&wallpaper_path)?;

    println!("Wallpaper set to: {}", name);

    Ok(())
}

/// Set a random wallpaper from the collection
fn set_random_wallpaper() -> Result<()> {
    let wallpaper_dir = config::get_wallpaper_dir()?;

    if !wallpaper_dir.exists() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    println!("Selecting random wallpaper...");

    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut wallpapers = Vec::new();

    for entry in WalkDir::new(&wallpaper_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if image_extensions.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str()) {
                    wallpapers.push(path.to_path_buf());
                }
            }
        }
    }

    if wallpapers.is_empty() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    let mut rng = rand::thread_rng();
    let chosen = wallpapers
        .choose(&mut rng)
        .context("Failed to choose random wallpaper")?;

    set_wallpaper_internal(chosen)?;

    if let Some(filename) = chosen.file_name() {
        println!("Random wallpaper set to: {}", filename.to_string_lossy());
    }

    Ok(())
}

fn set_wallpaper_internal(path: &Path) -> Result<()> {
    let absolute_path = fs::canonicalize(path).context("Failed to get absolute path")?;

    wallpaper::set_from_path(absolute_path.to_str().unwrap())
        .map_err(|e| anyhow::anyhow!("Failed to set wallpaper: {}", e))?;

    Ok(())
}
