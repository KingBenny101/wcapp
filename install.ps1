# wcapp Windows installer script
# Downloads and installs the latest wcapp binary

$ErrorActionPreference = "Stop"

Write-Host "wcapp Installer for Windows" -ForegroundColor Cyan
Write-Host ""

# Download URL
$url = "https://github.com/KingBenny101/wcapp/releases/latest/download/wcapp-windows-x86_64.exe"
$output = "$env:TEMP\wcapp.exe"

Write-Host "Downloading wcapp..." -ForegroundColor Yellow
try {
    Invoke-WebRequest -Uri $url -OutFile $output -UseBasicParsing
    Write-Host "✓ Download complete" -ForegroundColor Green
}
catch {
    Write-Host "✗ Failed to download: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Where would you like to install wcapp?" -ForegroundColor Yellow
Write-Host "1. Current directory (no admin required)"
Write-Host "2. C:\Program Files\wcapp (requires admin, adds to PATH)"
Write-Host "3. Custom location"
Write-Host ""

$choice = Read-Host "Enter choice (1-3)"

switch ($choice) {
    "1" {
        $installPath = ".\wcapp.exe"
        Copy-Item $output $installPath -Force
        Write-Host ""
        Write-Host "✓ wcapp installed to current directory" -ForegroundColor Green
        Write-Host ""
        Write-Host "Run with: .\wcapp" -ForegroundColor Cyan
    }
    "2" {
        $installDir = "C:\Program Files\wcapp"
        $installPath = "$installDir\wcapp.exe"
        
        # Check if running as admin
        $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
        
        if (-not $isAdmin) {
            Write-Host ""
            Write-Host "✗ Administrator privileges required for this option" -ForegroundColor Red
            Write-Host "Please run PowerShell as Administrator and try again" -ForegroundColor Yellow
            exit 1
        }
        
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
            Write-Host "✓ wcapp installed to $installPath" -ForegroundColor Green
            Write-Host "✓ Added to system PATH" -ForegroundColor Green
            Write-Host ""
            Write-Host "Please restart your terminal or run:" -ForegroundColor Yellow
            Write-Host "  `$env:Path = [System.Environment]::GetEnvironmentVariable('Path','Machine')" -ForegroundColor Cyan
        }
        else {
            Write-Host ""
            Write-Host "✓ wcapp installed to $installPath" -ForegroundColor Green
        }
    }
    "3" {
        $customPath = Read-Host "Enter full path (e.g., C:\tools\wcapp.exe)"
        $customDir = Split-Path $customPath -Parent
        
        if (-not (Test-Path $customDir)) {
            New-Item -ItemType Directory -Path $customDir -Force | Out-Null
        }
        
        Copy-Item $output $customPath -Force
        Write-Host ""
        Write-Host "✓ wcapp installed to $customPath" -ForegroundColor Green
        Write-Host ""
        Write-Host "Make sure the directory is in your PATH to run from anywhere" -ForegroundColor Yellow
    }
    default {
        Write-Host ""
        Write-Host "✗ Invalid choice" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "Installation complete! Run 'wcapp --help' to get started." -ForegroundColor Green
