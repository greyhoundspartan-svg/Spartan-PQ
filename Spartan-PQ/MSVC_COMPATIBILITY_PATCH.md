# MSVC Compatibility Patch for Labrador Library

## Problem Areas

### 1. GCC Attributes
The Labrador library uses GCC-specific attributes:
```c
__attribute__((visibility("hidden")))
```

**MSVC Equivalent:**
```c
__declspec(dllexport)  // For exported functions
// or simply remove the attribute for internal functions
```

### 2. Assembly Files
The `.S` assembly files use GCC syntax:
- `ntt.S`
- `invntt.S`

**MSVC Equivalent:**
- `.asm` files with MASM syntax
- Or use intrinsics instead of assembly

### 3. GCC-Specific Headers
- `#include <malloc.h>` (non-standard)
- GCC-specific builtins

## Patch Strategy

### Option A: Minimal Patch (Quick Fix)
1. **Comment out problematic attributes** in headers
2. **Exclude assembly files** from MSVC builds
3. **Replace malloc.h** with standard malloc

### Option B: Full Port (Extensive)
1. **Rewrite assembly in C intrinsics**
2. **Replace all GCC attributes**
3. **Standardize all headers**

### Option C: Conditional Compilation (Recommended)
```c
#ifdef _MSC_VER
  // MSVC-specific code
  #define ATTRIBUTE_HIDDEN
#else
  // GCC-specific code  
  #define ATTRIBUTE_HIDDEN __attribute__((visibility("hidden")))
#endif
```

## Implementation

To implement Option A (minimal patch):

1. **Modify poly.h:**
   ```c
   // Comment out: __attribute__((visibility("hidden")))
   ```

2. **Modify build.rs:**
   ```rust
   // Exclude assembly files for MSVC
   if !cfg!(target_env = "msvc") {
       build.file(labrador_dir.join("ntt.S"));
       build.file(labrador_dir.join("invntt.S"));
   }
   ```

3. **Add MSVC-specific defines:**
   ```rust
   build.define("_CRT_SECURE_NO_WARNINGS", "");
   ```

This would allow basic Greyhound functionality on MSVC, though with reduced performance.
