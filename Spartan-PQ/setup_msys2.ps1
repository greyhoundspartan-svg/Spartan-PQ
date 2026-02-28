# MSYS2 Setup Script for Spartan2
# Run this script in PowerShell as Administrator

Write-Host "Setting up MSYS2 for Spartan2..." -ForegroundColor Green

# Check if MSYS2 is installed
$msys2Path = "C:\msys64\msys2.exe"
if (-not (Test-Path $msys2Path)) {
    Write-Host "MSYS2 not found at $msys2Path" -ForegroundColor Red
    Write-Host "Please install MSYS2 from https://www.msys2.org/" -ForegroundColor Yellow
    exit 1
}

Write-Host "MSYS2 found. Installing required packages..." -ForegroundColor Green

# Install required packages
& $msys2Path -ucrt64 -c "pacman -Syu --noconfirm"
& $msys2Path -ucrt64 -c "pacman -S --needed --noconfirm mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-make"

Write-Host "MSYS2 setup complete!" -ForegroundColor Green
Write-Host "You can now run: cargo build" -ForegroundColor Cyan
