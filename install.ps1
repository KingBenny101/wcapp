# wcapp Windows installer script
# Downloads and installs the latest wcapp binary

$ErrorActionPreference = "Stop"

Write-Host "wcapp Installer"
Write-Host ""

# Detect architecture
$arch = $env:PROCESSOR_ARCHITECTURE
if ($arch -eq "AMD64") {
    $url = "https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-x86_64-pc-windows-msvc.exe"
}
elseif ($arch -eq "ARM64") {
    $url = "https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-aarch64-pc-windows-msvc.exe"
}
else {
    Write-Host "✗ Unsupported architecture: $arch"
    exit 1
}

Write-Host "Detected: Windows $arch"
Write-Host ""
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
Write-Host "1. $env:LOCALAPPDATA\Programs\wcapp (recommended, user install)"
Write-Host "2. Current directory"
Write-Host "3. Custom location"
Write-Host ""

$choice = Read-Host "Enter choice (1-3) [1]"

# Default to option 1 if empty
if (-not $choice) {
    $choice = "1"
}

switch ($choice) {
    "1" {
        $installDir = "$env:LOCALAPPDATA\Programs\wcapp"
        $installPath = "$installDir\wcapp.exe"
        
        # Create directory
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }
        
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
    "2" {
        $installPath = ".\wcapp.exe"
        Copy-Item $output $installPath -Force
        Write-Host ""
        Write-Host "✓ wcapp downloaded to current directory"
        Write-Host ""
        Write-Host "Run with: .\wcapp"
    }
    "3" {
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
