use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

use crate::config;

/// Remove all downloaded wallpapers with confirmation
pub fn execute() -> Result<()> {
    let wallpaper_dir = config::get_wallpaper_dir()?;

    if !wallpaper_dir.exists() {
        println!(
            "No wallpaper directory found at {}",
            wallpaper_dir.display()
        );
        return Ok(());
    }

    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut count = 0;

    for entry in WalkDir::new(&wallpaper_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if image_extensions.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str()) {
                    count += 1;
                }
            }
        }
    }

    if count == 0 {
        println!("No wallpapers found in {}", wallpaper_dir.display());
        return Ok(());
    }

    println!(
        "This will delete {} wallpapers from {}",
        count,
        wallpaper_dir.display()
    );
    println!("Are you sure? (y/N): ");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read user input")?;

    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("Operation cancelled");
        return Ok(());
    }

    fs::remove_dir_all(&wallpaper_dir).context("Failed to remove wallpaper directory")?;

    println!(
        "Deleted {} wallpapers from {}",
        count,
        wallpaper_dir.display()
    );

    Ok(())
}
