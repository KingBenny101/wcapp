mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        /// Destination folder (defaults to Pictures/wcapp)
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
        Commands::Fetch { destination } => commands::fetch::execute(destination)?,
        Commands::Set { name, random } => commands::set::execute(name, random)?,
        Commands::List => commands::list::execute()?,
        Commands::Clean => commands::clean::execute()?,
        Commands::Update => commands::update::execute()?,
    }

    Ok(())
}
