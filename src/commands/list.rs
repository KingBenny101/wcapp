use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

use crate::config;

/// List all available wallpapers
pub fn execute() -> Result<()> {
    let wallpaper_dir = config::get_wallpaper_dir()?;

    if !wallpaper_dir.exists() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    println!("Available wallpapers in {}:", wallpaper_dir.display());
    println!();

    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut total_count = 0;

    let mut categories = Vec::new();
    for entry in fs::read_dir(&wallpaper_dir).context("Failed to read wallpaper directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            categories.push(path);
        }
    }

    if categories.is_empty() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    categories.sort();

    for category_path in categories {
        let category_name = category_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let mut wallpapers = Vec::new();

        for entry in WalkDir::new(&category_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if image_extensions
                        .contains(&ext.to_str().unwrap_or("").to_lowercase().as_str())
                    {
                        if let Ok(relative) = path.strip_prefix(&wallpaper_dir) {
                            wallpapers.push(relative.display().to_string());
                        }
                    }
                }
            }
        }

        if !wallpapers.is_empty() {
            println!("[{}]", category_name);
            wallpapers.sort();
            for wallpaper in &wallpapers {
                println!("  - {}", wallpaper);
            }
            println!();
            total_count += wallpapers.len();
        }
    }

    println!("Total: {} wallpapers", total_count);

    Ok(())
}
