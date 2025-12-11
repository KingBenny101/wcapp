# wcapp Windows installer script
# Downloads and installs the latest wcapp binary

$ErrorActionPreference = "Stop"

Write-Host "wcapp Installer"
Write-Host ""

# Download URL
$url = "https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-windows-x86_64.exe"
$output = "$env:TEMP\wcapp.exe"

Write-Host "Downloading wcapp..."
try {
    Invoke-WebRequest -Uri $url -OutFile $output -UseBasicParsing
    Write-Host "✓ Download complete"
}
catch {
    Write-Host "✗ Failed to download: $_"
    exit 1
}

Write-Host ""
Write-Host "Where would you like to install wcapp?"
Write-Host "1. C:\Program Files\wcapp (recommended, requires admin, adds to PATH)"
Write-Host "2. $env:LOCALAPPDATA\Programs\wcapp (user install, no admin needed)"
Write-Host "3. Current directory"
Write-Host "4. Custom location"
Write-Host ""

$choice = Read-Host "Enter choice (1-4)"

switch ($choice) {
    "1" {
        # Check if running as admin
        $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
        
        if (-not $isAdmin) {
            Write-Host ""
            Write-Host "✗ Administrator privileges required for this option"
            Write-Host "Please run PowerShell as Administrator and try again"
            exit 1
        }
        
        $installDir = "C:\Program Files\wcapp"
        $installPath = "$installDir\wcapp.exe"
        
        # Create directory
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }
        
        # Copy binary
        Copy-Item $output $installPath -Force
        
        # Add to PATH
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
        if ($currentPath -notlike "*$installDir*") {
            [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "Machine")
            Write-Host ""
            Write-Host "✓ wcapp installed to $installPath"
            Write-Host "✓ Added to system PATH"
            Write-Host ""
            Write-Host "Please restart your terminal or run:"
            Write-Host "  `$env:Path = [System.Environment]::GetEnvironmentVariable('Path','Machine') + ';' + [System.Environment]::GetEnvironmentVariable('Path','User')"
        }
        else {
            Write-Host ""
            Write-Host "✓ wcapp installed to $installPath"
        }
    }
    "2" {
        $installDir = "$env:LOCALAPPDATA\Programs\wcapp"
        $installPath = "$installDir\wcapp.exe"
        
        # Create directory
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }
        
        # Copy binary
        Copy-Item $output $installPath -Force
        
        # Add to user PATH
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($currentPath -notlike "*$installDir*") {
            [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
            Write-Host ""
            Write-Host "✓ wcapp installed to $installPath"
            Write-Host "✓ Added to user PATH"
            Write-Host ""
            Write-Host "Please restart your terminal or run:"
            Write-Host "  `$env:Path = [System.Environment]::GetEnvironmentVariable('Path','Machine') + ';' + [System.Environment]::GetEnvironmentVariable('Path','User')"
        }
        else {
            Write-Host ""
            Write-Host "✓ wcapp installed to $installPath"
        }
    }
    "3" {
        $installPath = ".\wcapp.exe"
        Copy-Item $output $installPath -Force
        Write-Host ""
        Write-Host "✓ wcapp downloaded to current directory"
        Write-Host ""
        Write-Host "Run with: .\wcapp"
        Write-Host "To install globally later, move to a directory in PATH"
    }
    "4" {
        $customPath = Read-Host "Enter full path (e.g., C:\tools\wcapp.exe)"
        $customDir = Split-Path $customPath -Parent
        
        if (-not (Test-Path $customDir)) {
            Write-Host "Creating directory: $customDir"
            New-Item -ItemType Directory -Path $customDir -Force | Out-Null
        }
        
        Copy-Item $output $customPath -Force
        Write-Host ""
        Write-Host "✓ wcapp installed to $customPath"
        Write-Host ""
        Write-Host "Make sure $customDir is in your PATH to run from anywhere"
    }
    default {
        Write-Host ""
        Write-Host "✗ Invalid choice"
        exit 1
    }
}

Write-Host ""
Write-Host "Installation complete! Run 'wcapp --help' to get started."
