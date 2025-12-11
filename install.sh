#!/bin/sh
# wcapp installer script
# Downloads and installs the latest wcapp binary for your platform

set -e

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux*)
        if [ "$ARCH" = "x86_64" ]; then
            BINARY="wcapp-linux-x86_64"
        else
            echo "Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    Darwin*)
        if [ "$ARCH" = "arm64" ]; then
            BINARY="wcapp-macos-aarch64"
        elif [ "$ARCH" = "x86_64" ]; then
            BINARY="wcapp-macos-x86_64"
        else
            echo "Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "For Windows, please download from: https://github.com/KingBenny101/wcapp/releases/latest"
        exit 1
        ;;
esac

echo "Detected: $OS $ARCH"
echo "Downloading wcapp..."

# Download the binary
DOWNLOAD_URL="https://github.com/KingBenny101/wcapp/releases/latest/download/$BINARY"
curl -L "$DOWNLOAD_URL" -o wcapp

# Make it executable
chmod +x wcapp

# Try to move to /usr/local/bin
if [ -w /usr/local/bin ]; then
    mv wcapp /usr/local/bin/wcapp
    echo "✓ wcapp installed to /usr/local/bin/wcapp"
else
    echo "✓ wcapp downloaded to current directory"
    echo ""
    echo "To install globally, run:"
    echo "  sudo mv wcapp /usr/local/bin/wcapp"
    echo ""
    echo "Or add the current directory to your PATH"
fi

echo ""
echo "Installation complete! Run 'wcapp --help' to get started."
