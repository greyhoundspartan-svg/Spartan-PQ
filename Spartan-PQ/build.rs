

use std::path::PathBuf;

fn main() {
    // Declare the cfg flag for Rust
    println!("cargo::rustc-check-cfg=cfg(stub_greyhound)");
    println!("cargo::rustc-check-cfg=cfg(msvc_greyhound)");
    
    // Tell cargo to link the Greyhound C library
    let labrador_dir = PathBuf::from("../labrador-main");
    
    // Check if labrador directory exists
    if !labrador_dir.exists() {
        panic!("Labrador directory not found at {}. Please ensure labrador-main exists in the parent directory.", labrador_dir.display());
    }
    
    println!("cargo:rustc-link-search=native={}", labrador_dir.display());
    println!("cargo:rustc-link-lib=static=dogs");
    
    // Rebuild if C sources change
    println!("cargo:rerun-if-changed={}/greyhound.c", labrador_dir.display());
    println!("cargo:rerun-if-changed={}/greyhound.h", labrador_dir.display());
    println!("cargo:rerun-if-changed={}/labrador.c", labrador_dir.display());
    println!("cargo:rerun-if-changed={}/labrador.h", labrador_dir.display());
    
    // Debug info
    println!("cargo:warning=Building with target_env: {}", std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_else(|_| "unknown".to_string()));
    println!("cargo:warning=Building with target_arch: {}", std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "unknown".to_string()));
    
    // Compile the C library (or a stub on MSVC)
    let mut build = cc::Build::new();
    
    // Determine if we can use GCC
    let use_gcc = detect_and_configure_gcc(&mut build);
    
    // Select files based on compiler availability
    let c_files = if use_gcc {
        // Full set of files for GCC/Clang builds
        vec![
            labrador_dir.join("pack.c"),
            labrador_dir.join("greyhound.c"),
            labrador_dir.join("dachshund.c"),
            labrador_dir.join("chihuahua.c"),
            labrador_dir.join("labrador.c"),
            labrador_dir.join("data.c"),
            labrador_dir.join("jlproj.c"),
            labrador_dir.join("polx.c"),
            labrador_dir.join("poly.c"),
            labrador_dir.join("polz.c"),
            labrador_dir.join("sparsemat.c"),
            labrador_dir.join("aesctr.c"),
            labrador_dir.join("fips202.c"),
            labrador_dir.join("randombytes.c"),
        ]
    } else {
        // MSVC: Labrador C code relies on GCC/Clang extensions (C99 VLAs, __attribute__, complex),
        // so we build a minimal stub library instead. At the Rust level we use StubPCS.
        vec![PathBuf::from("msvc_stub.c")]
    };
    
    build.files(&c_files);
    if use_gcc {
        build.include(&labrador_dir);
    }
    
    // Add assembly files only if using GCC and on x86_64
    if use_gcc && cfg!(target_arch = "x86_64") {
        build
            .file(labrador_dir.join("ntt.S"))
            .file(labrador_dir.join("invntt.S"));
    }
    
    build.compile("dogs");
}

fn detect_and_configure_gcc(build: &mut cc::Build) -> bool {
    // On Windows with MSVC target, use a stub library (full Labrador requires GCC/Clang)
    if cfg!(target_env = "msvc") {
        println!("cargo:warning=Using MSVC - building stub library (full Greyhound requires GCC)");
        println!("cargo:rustc-cfg=msvc_greyhound");
        println!("cargo:rustc-cfg=stub_greyhound");
        build
            .flag("/W4")
            .flag("/O2")
            .define("_CRT_SECURE_NO_WARNINGS", "")
            .flag("/w"); // Suppress all warnings
        false
    } else {
        // Non-Windows - use GCC/Clang
        build
            .flag("-std=c99")
            .flag("-O3")
            .flag("-w");
        true
    }
}

