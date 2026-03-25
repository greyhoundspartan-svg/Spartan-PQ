# Greyhound Integration Summary

This document summarizes the integration of Greyhound (network-based polynomial commitment scheme) into Spartan2, replacing the Hyrax polynomial commitment scheme.

## Changes Made

### 1. FFI Bindings (`Spartan2-main/src/provider/pcs/greyhound_ffi.rs`)
- Created Rust FFI bindings for Greyhound C functions
- Defined structures matching the C API (`Polcomctx`, `Polcomprf`, `Witness`, `Prncplstmnt`)
- Exported necessary functions from the C library

### 2. Greyhound PCS Implementation (`Spartan2-main/src/provider/pcs/greyhound_pc.rs`)
- Implemented `PCSEngineTrait` for `GreyhoundPCS`
- Created types:
  - `GreyhoundCommitmentKey`: Commitment key structure
  - `GreyhoundVerifierKey`: Verifier key structure
  - `GreyhoundCommitment`: Commitment structure (stores hash)
  - `GreyhoundBlind`: Blind structure (for compatibility)
  - `GreyhoundEvaluationArgument`: Evaluation proof structure
- Implemented all required trait methods:
  - `setup`: Initialize commitment parameters
  - `blind`: Generate blinding values
  - `commit`: Commit to polynomial vectors
  - `check_commitment`: Validate commitment structure
  - `rerandomize_commitment`: Rerandomize commitments
  - `combine_commitments`: Combine multiple commitments
  - `combine_blinds`: Combine multiple blinds
  - `prove`: Generate evaluation proofs
  - `verify`: Verify evaluation proofs

### 3. Module Updates
- **`Spartan2-main/src/provider/pcs/mod.rs`**: Added `greyhound_pc` module export
- **`Spartan2-main/src/provider/mod.rs`**: 
  - Replaced `HyraxPCS` with `GreyhoundPCS` in engine definitions
  - Renamed engines:
    - `PallasHyraxEngine` → `PallasGreyhoundEngine`
    - `VestaHyraxEngine` → `VestaGreyhoundEngine`
    - `P256HyraxEngine` → `P256GreyhoundEngine`
    - `T256HyraxEngine` → `T256GreyhoundEngine`
  - Added deprecated aliases for backward compatibility

### 4. Build Configuration (`Spartan2-main/build.rs`)
- Created build script to compile Greyhound C library
- Added all necessary C source files:
  - `greyhound.c`, `dachshund.c`, `chihuahua.c`, `labrador.c`
  - `data.c`, `jlproj.c`, `polx.c`, `poly.c`, `polz.c`
  - `sparsemat.c`, `aesctr.c`, `fips202.c`, `randombytes.c`, `pack.c`
- Added assembly files (`ntt.S`, `invntt.S`) for x86_64
- Configured compiler flags and includes

### 5. Dependencies (`Spartan2-main/Cargo.toml`)
- Added `cc` crate as build dependency for compiling C code

### 6. Example Update (`Spartan2-main/examples/sha256.rs`)
- Updated to use `T256GreyhoundEngine` instead of `T256HyraxEngine`

## Architecture Differences

### Hyrax vs Greyhound

**Hyrax:**
- Based on discrete logarithm assumptions (elliptic curve groups)
- Uses group elements (`E::GE`) for commitments
- Works directly with scalar field elements (`E::Scalar`)
- Uses Inner Product Arguments (IPA) for proofs

**Greyhound:**
- Based on lattice assumptions (polynomial rings)
- Uses polynomial ring elements (`polz`, `polx`, `poly`)
- Works with polynomial vectors over rings
- Uses network-based commitment schemes
- Requires conversion between scalar fields and polynomial rings

## Implementation Status

### Completed
- FFI bindings structure
- Trait implementation structure
- Module organization
- Build configuration
- Engine renaming

###  Needs Refinement
The following areas need proper implementation:

1. **Field-to-Polynomial Conversion** (`commit` method):
   - Currently uses simplified conversion
   - Needs proper encoding from `E::Scalar` to `polz` representation
   - Must handle polynomial ring arithmetic correctly

2. **Polynomial Evaluation** (`prove` method):
   - Currently uses placeholder evaluation
   - Needs proper multilinear polynomial evaluation
   - Must convert evaluation points correctly

3. **Proof Generation** (`prove` method):
   - Currently returns empty proof data
   - Needs to call `polcom_eval` and `polcom_reduce` properly
   - Must manage memory correctly (C allocations)

4. **Proof Verification** (`verify` method):
   - Currently performs basic checks only
   - Needs to reconstruct statement and call `principle_verify`
   - Must deserialize proof data correctly

5. **Memory Management**:
   - C structures need proper allocation/deallocation
   - Must handle `_aligned_alloc` and `free` calls correctly
   - Need to ensure no memory leaks

## Next Steps

1. **Implement Proper Conversions**:
   - Create conversion functions between scalar fields and polynomial rings
   - Handle different field sizes and polynomial degrees
   - Ensure correctness of conversions

2. **Complete Proof Generation**:
   - Implement proper `polcom_eval` calls
   - Implement proper `polcom_reduce` calls
   - Serialize/deserialize proof structures correctly

3. **Complete Proof Verification**:
   - Implement statement reconstruction
   - Call `principle_verify` with correct parameters
   - Handle verification errors properly

4. **Testing**:
   - Create unit tests for conversion functions
   - Test commitment generation
   - Test proof generation and verification
   - Compare with original Hyrax implementation

5. **Performance Optimization**:
   - Optimize conversion routines
   - Minimize memory allocations
   - Profile and optimize hot paths

## Notes

- The integration maintains backward compatibility through deprecated type aliases
- The C library must be compiled before building the Rust code
- Assembly files are only included for x86_64 architectures
- The implementation currently uses simplified placeholders for complex operations
- Proper error handling and memory management are critical for production use

## References

- Greyhound C implementation: `labrador-main/greyhound.c`
- Greyhound header: `labrador-main/greyhound.h`
- Spartan2 PCS trait: `Spartan2-main/src/traits/pcs.rs`
- Original Hyrax implementation: `Spartan2-main/src/provider/pcs/hyrax_pc.rs`

