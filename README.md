# wcapp

Application for @Incalculas's wallpaper collection.

A cross-platform CLI tool to fetch and set wallpapers from the Incalculas wallpaper collection.

**Features:**

- Automatically fetches wallpapers from the configured repository
- Preserves folder structure and categories
- Set wallpapers by name or randomly
- Remembers your preferred wallpaper directory
- Works on Windows, macOS, and Linux

## Installation

### Option 1: Download Pre-built Binary (Easiest)

Download the latest release for your platform from [GitHub Releases](https://github.com/KingBenny101/wcapp/releases/latest):

**Windows:**

1. Download `wcapp-windows-x86_64.exe`
2. Rename to `wcapp.exe` (optional)
3. Move to a directory in your PATH, or run from current location

**macOS:**

```bash
# Intel Mac
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-macos-x86_64 -o wcapp
chmod +x wcapp
sudo mv wcapp /usr/local/bin/

# Apple Silicon (M1/M2)
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-macos-aarch64 -o wcapp
chmod +x wcapp
sudo mv wcapp /usr/local/bin/
```

**Linux:**

```bash
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-linux-x86_64 -o wcapp
chmod +x wcapp
sudo mv wcapp /usr/local/bin/
```

### Option 2: Install with Cargo (For Rust Users)

```bash
cargo install --git https://github.com/KingBenny101/wcapp
```

This installs `wcapp` globally and makes it available from anywhere in your terminal.

### Option 3: Build from Source

Clone the repository and build:

```bash
git clone https://github.com/KingBenny101/wcapp
cd wcapp
cargo build --release
```

The executable will be at `target/release/wcapp` (or `wcapp.exe` on Windows).

**Requirements:**

- Git (for fetching wallpapers - required for all options)
- Rust toolchain (only for Options 2 & 3 - install from [rustup.rs](https://rustup.rs))

## Commands

### Fetch Wallpapers

Download wallpapers from the Incalculas collection:

```bash
# Download to default location (Pictures/wcapp)
wcapp fetch

# Download to custom location
wcapp fetch --destination "C:\MyWallpapers"
# Or on macOS/Linux:
wcapp fetch --destination ~/MyWallpapers
```

**What it does:**

- Clones https://github.com/Incalculas/wallpapers
- Copies images from the `classified/` folder, preserving category structure
- Saves wallpapers to your chosen directory
- Remembers the directory for future commands

### List Available Wallpapers

See all downloaded wallpapers organized by category:

```bash
wcapp list
```

Output example:

```
Available wallpapers in C:\Users\YourName\Pictures\wcapp:

[Nature]
  - Nature/beach-sunset.jpg
  - Nature/mountains.jpg

[Abstract]
  - Abstract/geometric.jpg

Total: 3 wallpapers
```

### Set a Specific Wallpaper

Set a wallpaper by its path (include category):

```bash
wcapp set --name "Nature/sunset.jpg"
```

If you don't specify a name, it will list available wallpapers:

```bash
wcapp set
```

### Set Random Wallpaper

Let the app choose a random wallpaper:

```bash
wcapp set --random
```

### Remove All Wallpapers

Delete all downloaded wallpapers (requires confirmation):

```bash
wcapp clean
```

**What it does:**

- Counts all wallpapers in your collection
- Shows the total and directory location
- Asks for confirmation before deletion
- Removes the entire wallpaper directory

Example interaction:

```
$ wcapp clean
This will delete 156 wallpapers from C:\Users\YourName\Pictures\wcapp
Are you sure? (y/N): y
Successfully removed 156 wallpapers
```

## Examples

```bash
# First time: fetch wallpapers to default location
wcapp fetch

# See what's available
wcapp list

# Set a specific wallpaper
wcapp set --name "Nature/beach.jpg"

# Set a random wallpaper
wcapp set --random

# Remove all wallpapers (with confirmation)
wcapp clean

# Use custom directory (remembered for all future commands)
wcapp fetch --destination ~/MyWallpapers
wcapp list  # Now looks in ~/MyWallpapers
```

## Configuration

The app remembers your wallpaper directory in a config file:

- **Windows**: `%APPDATA%\wcapp\config.toml`
- **macOS**: `~/Library/Application Support/wcapp/config.toml`
- **Linux**: `~/.config/wcapp/config.toml`

Example config:

```toml
wallpaper_dir = "C:\\Users\\YourName\\Pictures\\wcapp"
```

## Details

- **Wallpaper repository**: https://github.com/Incalculas/wallpapers (hardcoded)
- **Default wallpaper directory by OS:**
  - **Windows**: `%USERPROFILE%\Pictures\wcapp`
  - **macOS**: `~/Pictures/wcapp`
  - **Linux**: `~/Pictures/wcapp`
- **Category structure preserved**: The app maintains the `classified/` folder organization
- **Supported formats**: jpg, jpeg, png, bmp, gif, webp
- **Persistent settings**: Your chosen directory is remembered across sessions
