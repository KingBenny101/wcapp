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

### Quick Install

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.ps1 | iex
```

**macOS/Linux:**

```bash
curl -fsSL https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.sh | sh
```

These scripts automatically download and install the latest version for your platform.

### Manual Installation

#### Windows

1. Download `wcapp-windows-x86_64.exe` from [releases](https://github.com/KingBenny101/wcapp/releases/latest)
2. Rename to `wcapp.exe`
3. Add to PATH or run from current directory

#### macOS/Linux

Download for your platform:

**macOS (Intel):**

```bash
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-macos-x86_64 -o wcapp
chmod +x wcapp && sudo mv wcapp /usr/local/bin/
```

**macOS (Apple Silicon):**

```bash
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-macos-aarch64 -o wcapp
chmod +x wcapp && sudo mv wcapp /usr/local/bin/
```

**Linux:**

```bash
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-linux-x86_64 -o wcapp
chmod +x wcapp && sudo mv wcapp /usr/local/bin/
```

### Alternative Methods

**With Cargo (Rust required):**

```bash
cargo install --git https://github.com/KingBenny101/wcapp
```

**Build from Source:**

```bash
git clone https://github.com/KingBenny101/wcapp
cd wcapp
cargo build --release
# Binary at: target/release/wcapp
```

**Requirements:**

- Git (for fetching wallpapers)

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

Example interaction:

```
$ wcapp clean
This will delete 156 wallpapers from C:\Users\YourName\Pictures\wcapp
Are you sure? (y/N): y
Successfully removed 156 wallpapers
```

### Update wcapp

Update to the latest version:

```bash
wcapp update
```

The app checks for new versions and automatically downloads and installs the update.

### Uninstall wcapp

Remove wcapp from your system:

```bash
wcapp uninstall
```

You'll be prompted to choose what to remove:

1. Just the wcapp binary
2. Binary + configuration
3. Binary + configuration + wallpapers

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

# Update to latest version
wcapp update

# Uninstall wcapp
wcapp uninstall

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

- **Wallpaper repository**: https://github.com/Incalculas/wallpapers (hardcoded)
- **Default wallpaper directory by OS:**
  - **Windows**: `%USERPROFILE%\Pictures\wcapp`
  - **macOS**: `~/Pictures/wcapp`
  - **Linux**: `~/Pictures/wcapp`
