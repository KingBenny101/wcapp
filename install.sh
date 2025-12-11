#!/bin/sh
# wcapp installer script
# Downloads and installs the latest wcapp binary for your platform

set -e

echo "wcapp Installer"
echo ""

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux*)
        if [ "$ARCH" = "x86_64" ]; then
            BINARY="wcapp-linux-x86_64"
        else
            echo "✗ Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    Darwin*)
        if [ "$ARCH" = "arm64" ]; then
            BINARY="wcapp-macos-aarch64"
        elif [ "$ARCH" = "x86_64" ]; then
            BINARY="wcapp-macos-x86_64"
        else
            echo "✗ Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    *)
        echo "✗ Unsupported OS: $OS"
        echo "For Windows, please run:"
        echo "  irm https://raw.githubusercontent.com/KingBenny101/wcapp/master/install.ps1 | iex"
        exit 1
        ;;
esac

echo "Detected: $OS $ARCH"
echo ""

# Download the binary
DOWNLOAD_URL="https://github.com/KingBenny101/wcapp/releases/latest/download/$BINARY"
echo "Downloading wcapp..."

if command -v curl >/dev/null 2>&1; then
    curl -L "$DOWNLOAD_URL" -o wcapp
elif command -v wget >/dev/null 2>&1; then
    wget "$DOWNLOAD_URL" -O wcapp
else
    echo "✗ Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Make it executable
chmod +x wcapp

echo "✓ Download complete"
echo ""

# Installation options
echo "Where would you like to install wcapp?"
echo "1. /usr/local/bin (recommended, requires sudo)"
echo "2. ~/.local/bin (user install, no sudo needed)"
echo "3. Current directory"
echo "4. Custom location"
echo ""
printf "Enter choice (1-4): "
read -r choice

case "$choice" in
    1)
        if [ -w /usr/local/bin ]; then
            mv wcapp /usr/local/bin/wcapp
            echo ""
            echo "✓ wcapp installed to /usr/local/bin/wcapp"
        else
            sudo mv wcapp /usr/local/bin/wcapp
            echo ""
            echo "✓ wcapp installed to /usr/local/bin/wcapp"
        fi
        ;;
    2)
        INSTALL_DIR="$HOME/.local/bin"
        if [ ! -d "$INSTALL_DIR" ]; then
            mkdir -p "$INSTALL_DIR"
        fi
        
        mv wcapp "$INSTALL_DIR/wcapp"
        echo ""
        echo "✓ wcapp installed to $INSTALL_DIR/wcapp"
        
        # Check if ~/.local/bin is in PATH
        case ":$PATH:" in
            *":$INSTALL_DIR:"*) 
                ;;
            *)
                echo ""
                echo "Note: $INSTALL_DIR is not in your PATH"
                echo "Add this line to your shell config (~/.bashrc, ~/.zshrc, etc.):"
                echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
                ;;
        esac
        ;;
    3)
        echo ""
        echo "✓ wcapp downloaded to current directory"
        echo ""
        echo "Run with: ./wcapp"
        echo "To install globally later, run:"
        echo "  sudo mv wcapp /usr/local/bin/wcapp"
        ;;
    4)
        printf "Enter full path (e.g., /opt/bin/wcapp): "
        read -r custom_path
        custom_dir=$(dirname "$custom_path")
        
        if [ ! -d "$custom_dir" ]; then
            echo "Creating directory: $custom_dir"
            mkdir -p "$custom_dir"
        fi
        
        if [ -w "$custom_dir" ]; then
            mv wcapp "$custom_path"
        else
            sudo mv wcapp "$custom_path"
        fi
        
        echo ""
        echo "✓ wcapp installed to $custom_path"
        echo ""
        echo "Make sure $custom_dir is in your PATH to run from anywhere"
        ;;
    *)
        echo ""
        echo "✗ Invalid choice"
        rm wcapp
        exit 1
        ;;
esac

echo ""
echo "Installation complete! Run 'wcapp --help' to get started."
