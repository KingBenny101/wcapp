# wcapp

Application for @Incalculas's wallpaper collection.

A cross-platform CLI tool to fetch and set wallpapers from the Incalculas wallpaper collection.

> Not a single line of rust code was written by me. Purely vibecoded.

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

**Windows (x86_64/ARM64):**

1. Download the appropriate binary for your architecture from [releases](https://github.com/KingBenny101/wcapp/releases/latest)
2. Create directory: `%LOCALAPPDATA%\Programs\wcapp`
3. Place the binary there and rename to `wcapp.exe`
4. Add `%LOCALAPPDATA%\Programs\wcapp` to your user PATH

#### macOS/Linux

**macOS (Intel/Apple Silicon):**

```bash
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-x86_64-apple-darwin -o wcapp  # or aarch64-apple-darwin
chmod +x wcapp
mkdir -p ~/.local/bin
mv wcapp ~/.local/bin/
# Add to PATH if needed: export PATH="$HOME/.local/bin:$PATH"
```

**Linux (x86_64/ARM64/32-bit):**

```bash
curl -L https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-x86_64-unknown-linux-gnu -o wcapp  # or aarch64/i686 variants
chmod +x wcapp
mkdir -p ~/.local/bin
mv wcapp ~/.local/bin/
# Add to PATH if needed: export PATH="$HOME/.local/bin:$PATH"
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

### Cycle Wallpapers

Automatically cycle through wallpapers at a specified interval:

```bash
wcapp cycle --interval 300
```

The interval is in seconds (default: 300 seconds = 5 minutes). The command runs continuously, changing the wallpaper at the specified interval.

To save the interval as your default:

```bash
wcapp cycle --interval 600 --set-default
```

To use the saved default interval:

```bash
wcapp cycle
```

**Note**: This command runs in the foreground. Press Ctrl+C to stop cycling.

To run in the background:

- **Windows**: Use Task Scheduler or run in a detached PowerShell window
- **macOS/Linux**: Use `nohup wcapp cycle &` or run in a screen/tmux session

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

# Cycle through wallpapers every 10 minutes
wcapp cycle --interval 600

# Cycle with custom interval and save as default
wcapp cycle --interval 900 --set-default

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

The app remembers your wallpaper directory and cycle interval in a config file:

- **Windows**: `%APPDATA%\wcapp\config.toml`
- **macOS**: `~/Library/Application Support/wcapp/config.toml`
- **Linux**: `~/.config/wcapp/config.toml`

Example config:

```toml
wallpaper_dir = "C:\\Users\\YourName\\Pictures\\wcapp"
cycle_interval = 600  # seconds
```

- **Wallpaper repository**: https://github.com/Incalculas/wallpapers (hardcoded)
- **Default wallpaper directory by OS:**
  - **Windows**: `%USERPROFILE%\Pictures\wcapp`
  - **macOS**: `~/Pictures/wcapp`
  - **Linux**: `~/Pictures/wcapp`
