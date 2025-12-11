use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

use crate::config::{self, Config};

pub fn execute(interval: Option<u64>, set_default: bool) -> Result<()> {
    let wallpaper_dir = config::get_wallpaper_dir()?;

    if !wallpaper_dir.exists() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    let mut config_data = config::load_config();
    let cycle_interval = if let Some(interval) = interval {
        if set_default {
            if let Some(ref mut cfg) = config_data {
                cfg.cycle_interval = interval;
                config::save_config(cfg)?;
                println!("✓ Default cycle interval set to {} seconds", interval);
            } else {
                let cfg = Config {
                    wallpaper_dir: wallpaper_dir.clone(),
                    cycle_interval: interval,
                };
                config::save_config(&cfg)?;
                println!("✓ Default cycle interval set to {} seconds", interval);
            }
        }
        interval
    } else {
        config_data
            .as_ref()
            .map(|c| c.cycle_interval)
            .unwrap_or(300)
    };

    println!("Wallpaper Cycle Mode");
    println!("Interval: {} seconds ({} minutes)", cycle_interval, cycle_interval / 60);
    println!("Directory: {}", wallpaper_dir.display());
    println!();
    println!("Press Ctrl+C to stop");
    println!();

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
        println!("No wallpapers found in {}", wallpaper_dir.display());
        return Ok(());
    }

    println!("Found {} wallpapers", wallpapers.len());
    println!();

    let mut rng = rand::thread_rng();
    let mut cycle_count = 0;

    loop {
        cycle_count += 1;
        
        let chosen = wallpapers
            .choose(&mut rng)
            .context("Failed to choose random wallpaper")?;

        match set_wallpaper_internal(chosen) {
            Ok(_) => {
                let filename = chosen
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                let category = chosen
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str());
                
                let display_name = if let Some(cat) = category {
                    format!("{}/{}", cat, filename)
                } else {
                    filename.to_string()
                };
                
                let now = chrono::Local::now();
                println!("[{}] Cycle #{}: {}", 
                    now.format("%H:%M:%S"), 
                    cycle_count, 
                    display_name
                );
            }
            Err(e) => {
                eprintln!("Failed to set wallpaper: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(cycle_interval));
    }
}

fn set_wallpaper_internal(path: &PathBuf) -> Result<()> {
    let absolute_path = fs::canonicalize(path).context("Failed to get absolute path")?;
    wallpaper::set_from_path(absolute_path.to_str().unwrap())
        .map_err(|e| anyhow::anyhow!("Failed to set wallpaper: {}", e))?;
    Ok(())
}
