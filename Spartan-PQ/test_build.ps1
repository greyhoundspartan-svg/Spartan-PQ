# Test Script for Spartan2 Build
# Run this in PowerShell to verify everything works

Write-Host "Testing Spartan2 Build Environment..." -ForegroundColor Green

# Test 1: Check if Rust is installed
Write-Host "1. Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "   ✓ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "   ✗ Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Test 2: Check if Cargo is installed
Write-Host "2. Checking Cargo installation..." -ForegroundColor Yellow
try {
    $cargoVersion = cargo --version 2>$null
    Write-Host "   ✓ Cargo found: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "   ✗ Cargo not found." -ForegroundColor Red
    exit 1
}

# Test 3: Check if MSYS2 is installed
Write-Host "3. Checking MSYS2 installation..." -ForegroundColor Yellow
$msys2Paths = @(
    "C:\msys64\ucrt64\bin\gcc.exe",
    "C:\msys64\mingw64\bin\gcc.exe",
    "C:\msys64\clang64\bin\gcc.exe"
)

$gccFound = $false
foreach ($path in $msys2Paths) {
    if (Test-Path $path) {
        $version = & $path --version 2>$null | Select-Object -First 1
        Write-Host "   ✓ GCC found: $path" -ForegroundColor Green
        Write-Host "     $version" -ForegroundColor Cyan
        $gccFound = $true
        break
    }
}

if (-not $gccFound) {
    Write-Host "   ✗ GCC not found. Please install MSYS2 with UCRT64 toolchain:" -ForegroundColor Red
    Write-Host "     1. Download from https://www.msys2.org/" -ForegroundColor Yellow
    Write-Host "     2. Run: pacman -Syu" -ForegroundColor Yellow
    Write-Host "     3. Run: pacman -S mingw-w64-ucrt-x86_64-gcc" -ForegroundColor Yellow
    exit 1
}

# Test 4: Check if Labrador directory exists
Write-Host "4. Checking Labrador directory..." -ForegroundColor Yellow
if (Test-Path "..\labrador-main") {
    Write-Host "   ✓ Labrador directory found" -ForegroundColor Green
} else {
    Write-Host "   ✗ Labrador directory not found at ..\labrador-main" -ForegroundColor Red
    exit 1
}

# Test 5: Try to build
Write-Host "5. Attempting to build Spartan2..." -ForegroundColor Yellow
try {
    $buildOutput = cargo build 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✓ Build successful!" -ForegroundColor Green
    } else {
        Write-Host "   ✗ Build failed with errors:" -ForegroundColor Red
        Write-Host $buildOutput -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "   ✗ Build failed with exception: $_" -ForegroundColor Red
    exit 1
}

Write-Host "All tests passed! Spartan2 is ready to use." -ForegroundColor Green
Write-Host "You can now run: cargo run --example sha256" -ForegroundColor Cyan
