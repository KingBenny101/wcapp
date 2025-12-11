use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Hardcoded wallpaper repository URL
const WALLPAPER_REPO: &str = "https://github.com/Incalculas/wallpapers";

/// Configuration structure
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    wallpaper_dir: PathBuf,
}

/// A CLI tool to fetch and set wallpapers
#[derive(Parser, Debug)]
#[command(name = "wcapp")]
#[command(version, about = "Wallpaper Collection Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch wallpapers from the repository
    Fetch {
        /// Destination folder (defaults to Pictures/Wallpapers)
        #[arg(short, long)]
        destination: Option<PathBuf>,
    },
    /// Set a specific wallpaper
    Set {
        /// Name of the wallpaper file (you can use tab completion or list available ones)
        #[arg(short, long)]
        name: Option<String>,

        /// Set a random wallpaper instead
        #[arg(short, long)]
        random: bool,
    },
    /// List all available wallpapers
    List,
    /// Remove all downloaded wallpapers
    Clean,
    /// Update wcapp to the latest version
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { destination } => {
            fetch_wallpapers(destination)?;
        }
        Commands::Set { name, random } => {
            if random {
                set_random_wallpaper()?;
            } else if let Some(wallpaper_name) = name {
                set_wallpaper(&wallpaper_name)?;
            } else {
                println!("Please specify a wallpaper name with --name or use --random flag");
                list_wallpapers()?;
            }
        }
        Commands::List => {
            list_wallpapers()?;
        }
        Commands::Clean => {
            clean_wallpapers()?;
        }
        Commands::Update => {
            update_wcapp()?;
        }
    }

    Ok(())
}

/// Get the config file path
fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("wcapp");

    // Create config directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }

    Ok(config_dir.join("config.toml"))
}

/// Load config from file, or return None if it doesn't exist
fn load_config() -> Result<Option<Config>> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(None);
    }

    let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;

    let config: Config = toml::from_str(&config_str).context("Failed to parse config file")?;

    Ok(Some(config))
}

/// Save config to file
fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;

    let config_str = toml::to_string_pretty(config).context("Failed to serialize config")?;

    fs::write(&config_path, config_str).context("Failed to write config file")?;

    Ok(())
}

/// Get the default wallpaper directory (Pictures/wcapp)
fn get_default_wallpaper_dir() -> Result<PathBuf> {
    let pictures_dir = dirs::picture_dir().context("Could not find Pictures directory")?;

    Ok(pictures_dir.join("wcapp"))
}

/// Get the wallpaper directory (Pictures/wcapp or from config)
fn get_wallpaper_dir() -> Result<PathBuf> {
    // Try to load from config first
    if let Some(config) = load_config()? {
        return Ok(config.wallpaper_dir);
    }

    // Fall back to default
    get_default_wallpaper_dir()
}

/// Fetch wallpapers from a git repository and move images to destination
fn fetch_wallpapers(destination: Option<PathBuf>) -> Result<()> {
    println!("Fetching wallpapers from: {}", WALLPAPER_REPO);

    // Determine destination directory
    let dest_dir = destination
        .unwrap_or_else(|| get_wallpaper_dir().expect("Could not determine wallpaper directory"));

    println!("Destination: {}", dest_dir.display());

    // Create a temporary directory for cloning
    let temp_dir = std::env::temp_dir().join("wcapp_temp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // Clone the repository
    println!("Cloning repository...");
    let status = Command::new("git")
        .args(["clone", "--depth", "1", WALLPAPER_REPO])
        .arg(&temp_dir)
        .status()
        .context("Failed to execute git. Make sure git is installed and in PATH")?;

    if !status.success() {
        anyhow::bail!("Git clone failed");
    }

    // Find and copy all image files from the classified folder, preserving structure
    println!("Copying images with folder structure...");
    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut copied_count = 0;

    // Look for the classified folder in the repo
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
                    // Get the relative path from the classified folder
                    if let Ok(relative_path) = path.strip_prefix(&classified_dir) {
                        let dest_path = dest_dir.join(relative_path);

                        // Create parent directories if they don't exist
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

    // Clean up temporary directory
    fs::remove_dir_all(&temp_dir)?;

    println!(
        "Successfully copied {} wallpapers to {}",
        copied_count,
        dest_dir.display()
    );

    // Save the destination directory to config for future use
    let config = Config {
        wallpaper_dir: dest_dir.clone(),
    };
    save_config(&config)?;

    println!("Wallpaper directory saved to config");

    Ok(())
}

/// List all available wallpapers
fn list_wallpapers() -> Result<()> {
    let wallpaper_dir = get_wallpaper_dir()?;

    // Check if directory exists
    if !wallpaper_dir.exists() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    println!("Available wallpapers in {}:", wallpaper_dir.display());
    println!();

    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut total_count = 0;

    // Get all subdirectories (categories)
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

    // List wallpapers by category
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

/// Set a specific wallpaper by name
fn set_wallpaper(name: &str) -> Result<()> {
    let wallpaper_dir = get_wallpaper_dir()?;
    let wallpaper_path = wallpaper_dir.join(name);

    if !wallpaper_path.exists() {
        anyhow::bail!(
            "Wallpaper '{}' not found in {}",
            name,
            wallpaper_dir.display()
        );
    }

    set_wallpaper_windows(&wallpaper_path)?;

    println!("Wallpaper set to: {}", name);

    Ok(())
}

/// Set a random wallpaper from the collection
fn set_random_wallpaper() -> Result<()> {
    let wallpaper_dir = get_wallpaper_dir()?;

    // Check if directory exists
    if !wallpaper_dir.exists() {
        println!("No wallpapers found. Use 'fetch' command to download wallpapers first.");
        return Ok(());
    }

    println!("Selecting random wallpaper...");

    let image_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];
    let mut wallpapers = Vec::new();

    // Use WalkDir to recursively search through category folders
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

    set_wallpaper_windows(chosen)?;

    if let Some(filename) = chosen.file_name() {
        println!("Random wallpaper set to: {}", filename.to_string_lossy());
    }

    Ok(())
}

/// Cross-platform function to set wallpaper using wallpaper crate
fn set_wallpaper_windows(path: &Path) -> Result<()> {
    // Convert path to absolute path
    let absolute_path = fs::canonicalize(path).context("Failed to get absolute path")?;

    // Use wallpaper crate to set the wallpaper (works on Windows, macOS, and Linux)
    wallpaper::set_from_path(absolute_path.to_str().unwrap())
        .map_err(|e| anyhow::anyhow!("Failed to set wallpaper: {}", e))?;

    Ok(())
}

/// Remove all downloaded wallpapers with confirmation
fn clean_wallpapers() -> Result<()> {
    let wallpaper_dir = get_wallpaper_dir()?;

    if !wallpaper_dir.exists() {
        println!(
            "No wallpaper directory found at {}",
            wallpaper_dir.display()
        );
        return Ok(());
    }

    // Count wallpapers to show user what will be deleted
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

    // Ask for confirmation
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

    // Delete the entire wallpaper directory
    fs::remove_dir_all(&wallpaper_dir).context("Failed to remove wallpaper directory")?;

    println!("Successfully removed {} wallpapers", count);

    Ok(())
}

/// Update wcapp to the latest version
fn update_wcapp() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: {}", current_version);
    println!("Checking for updates...");
    println!();

    // Fetch latest release info from GitHub API
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

    // Remove 'v' prefix if present
    let latest_version = latest_version.trim_start_matches('v');

    if latest_version == current_version {
        println!(
            "You are already running the latest version ({}).",
            current_version
        );
        return Ok(());
    }

    // Compare versions
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
        println!("Downloading update script...");

        let status = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &format!("irm {} | iex", script_url),
            ])
            .status()
            .context("Failed to execute PowerShell. Make sure PowerShell is available.")?;

        if !status.success() {
            anyhow::bail!("Update failed");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let script_url = "https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.sh";
        println!("Downloading update script...");

        let status = Command::new("sh")
            .args(["-c", &format!("curl -fsSL {} | sh", script_url)])
            .status()
            .context("Failed to execute update script. Make sure curl and sh are available.")?;

        if !status.success() {
            anyhow::bail!("Update failed");
        }
    }

    println!();
    println!("Update complete! You may need to restart your terminal.");

    Ok(())
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
