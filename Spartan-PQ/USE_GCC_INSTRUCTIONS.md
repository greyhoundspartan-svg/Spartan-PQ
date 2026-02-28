# How to Get Full Greyhound PCS Working on Windows

## Option 1: Use GCC Toolchain (Recommended)

1. **Install MSYS2 with GCC:**
   ```powershell
   # Run the setup script
   .\setup_msys2.ps1
   ```

2. **Configure Rust to use GCC:**
   ```powershell
   # Install GNU toolchain for Rust
   rustup toolchain install stable-x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```

3. **Modify build.rs to force GCC:**
   Change the `detect_and_configure_gcc` function to always return `true` and use GCC flags.

4. **Rebuild:**
   ```powershell
   cargo clean
   cargo build
   ```

## Option 2: Use WSL (Windows Subsystem for Linux)

1. **Install WSL:**
   ```powershell
   wsl --install
   ```

2. **Install Ubuntu and build in WSL environment.**

## Option 3: Cross-compile from Linux

1. **Use a Linux machine or VM to build, then copy binaries to Windows.**

## Why MSVC Doesn't Work

The Labrador C library contains:
- GCC-specific attributes (`__attribute__`)
- GCC assembly syntax (`.S` files)
- GCC-specific compiler flags
- Linux/Unix-style headers

These are fundamentally incompatible with MSVC compiler.

## Current Stub Implementation

The stub PCS provides:
- ✅ Successful compilation on MSVC
- ✅ Proper trait implementations
- ✅ Clear error messages when ZK operations are attempted
- ❌ No actual zero-knowledge proof functionality

## Recommendation

Use **Option 1 (GCC toolchain)** for full Greyhound functionality while staying on Windows.
