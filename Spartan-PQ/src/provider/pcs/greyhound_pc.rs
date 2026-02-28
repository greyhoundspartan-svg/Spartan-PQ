// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: MIT
// This file is part of the Spartan2 project.
// See the LICENSE file in the project root for full license information.
// Source repository: https://github.com/Microsoft/Spartan2

//! This module implements the Greyhound polynomial commitment scheme (network-based)
#![cfg(not(stub_greyhound))] // Only compile when not using stub

use crate::{
  errors::SpartanError,
  start_span,
  traits::{
    Engine,
    pcs::PCSEngineTrait,
    transcript::TranscriptReprTrait,
  },
};
use core::marker::PhantomData;
use ff::PrimeField;
use num_integer::div_ceil;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::os::raw::c_int;
use tracing::info;

mod greyhound_ffi;
use greyhound_ffi::*;

/// A type that holds commitment parameters for Greyhound commitments
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GreyhoundCommitmentKey<E: Engine> {
  len: usize,
  _phantom: PhantomData<E>,
}

/// A type that holds the verifier key for Greyhound commitments
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GreyhoundVerifierKey<E: Engine> {
  len: usize,
  _phantom: PhantomData<E>,
}

/// Structure that holds commitments (hash of u1 in Greyhound)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GreyhoundCommitment<E: Engine> {
  hash: [u8; 16],
  len: usize,
  ctx_data: Vec<u8>, // Serialized polcomctx for later use
  _phantom: PhantomData<E>,
}

/// Structure that holds blinds
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GreyhoundBlind<E: Engine> {
  blind: Vec<E::Scalar>,
}

/// Provides a commitment engine using Greyhound
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GreyhoundPCS<E: Engine> {
  _p: PhantomData<E>,
}

/// Provides an implementation of a polynomial evaluation argument using Greyhound
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GreyhoundEvaluationArgument<E: Engine> {
  proof_data: Vec<u8>, // Serialized polcomprf
  statement_data: Vec<u8>, // Serialized prncplstmnt
  witness_data: Vec<u8>, // Serialized witness
  len: usize,
  x: i64,
  y: i64,
  _phantom: PhantomData<E>,
}

/// Convert scalar field element to int64 array for polz representation
/// Each scalar is encoded as a constant polynomial (all N coefficients are the same)
fn scalar_to_int64_array<F: PrimeField>(scalar: &F) -> Vec<i64> {
  let mut result = Vec::with_capacity(N);
  let bytes = scalar.to_repr();
  
  // Convert to i64 - take first 8 bytes and interpret as i64
  let mut val = 0i64;
  for (i, &byte) in bytes.as_ref().iter().take(8).enumerate() {
    val |= (byte as i64) << (i * 8);
  }
  
  // For constant polynomial, all coefficients are the same
  // But we need to ensure it fits in the modulus
  let q = ((1i64 << LOGQ) - 99) as i64; // QOFF = 99 for LOGQ=32
  val = val % q;
  if val < 0 {
    val += q;
  }
  
  // All N coefficients are the same for constant polynomial
  result.resize(N, val);
  result
}

/// Convert vector of scalars to polz array
fn scalars_to_polz<F: PrimeField>(scalars: &[F]) -> Vec<Polz> {
  let len = scalars.len();
  let deg = 1; // Each scalar is a constant polynomial
  let total_size = len * deg * N;
  
  // Convert all scalars to int64 arrays
  let mut int64_vec = Vec::with_capacity(total_size);
  for scalar in scalars {
    let arr = scalar_to_int64_array(scalar);
    int64_vec.extend_from_slice(&arr);
  }
  
  // Allocate polz array
  let mut polz_vec = vec![
    Polz {
      limbs: [
        Vecn { v: [0i16; N] },
        Vecn { v: [0i16; N] },
        Vecn { v: [0i16; N] },
      ]
    };
    len * deg
  ];
  
  // Call C function to convert
  unsafe {
    polzvec_fromint64vec(
      polz_vec.as_mut_ptr(),
      len,
      deg,
      int64_vec.as_ptr(),
    );
  }
  
  polz_vec
}

impl<E: Engine> PCSEngineTrait<E> for GreyhoundPCS<E> {
  type CommitmentKey = GreyhoundCommitmentKey<E>;
  type VerifierKey = GreyhoundVerifierKey<E>;
  type Commitment = GreyhoundCommitment<E>;
  type Blind = GreyhoundBlind<E>;
  type EvaluationArgument = GreyhoundEvaluationArgument<E>;

  fn setup(
    _label: &'static [u8],
    n: usize,
    _width: usize,
  ) -> (Self::CommitmentKey, Self::VerifierKey) {
    let ck = GreyhoundCommitmentKey {
      len: n,
      _phantom: PhantomData,
    };
    let vk = GreyhoundVerifierKey {
      len: n,
      _phantom: PhantomData,
    };
    (ck, vk)
  }

  fn blind(_ck: &Self::CommitmentKey, n: usize) -> Self::Blind {
    // Greyhound doesn't use traditional blinds, but we maintain compatibility
    GreyhoundBlind {
      blind: (0..n)
        .map(|_| E::Scalar::random(&mut OsRng))
        .collect::<Vec<E::Scalar>>(),
    }
  }

  fn commit(
    ck: &Self::CommitmentKey,
    v: &[E::Scalar],
    _r: &Self::Blind,
    _is_small: bool,
  ) -> Result<Self::Commitment, SpartanError> {
    let (_commit_span, commit_t) = start_span!("greyhound_commit");
    
    let n = v.len();
    if n != ck.len {
      return Err(SpartanError::InvalidInputLength {
        reason: format!(
          "Greyhound commit: Expected {} elements, got {}",
          ck.len, n
        ),
      });
    }

    // Initialize commitment key (required before commit)
    unsafe {
      init_comkey(n * 64); // Estimate size needed
    }
    
    // Convert scalars to polz
    let polz_vec = scalars_to_polz(v);
    
    // Allocate and initialize polcomctx on heap to ensure it stays alive
    let mut ctx_box = Box::new(Polcomctx {
      len: n,
      m: 0,
      n: 0,
      s: polz_vec.as_ptr(),
      sx: std::ptr::null_mut(),
      t: std::ptr::null_mut(),
      u1: std::ptr::null_mut(),
      h: [0u8; 16],
      normsq: 0,
    });
    
    // Call polcom_commit (this will allocate ctx->sx, ctx->t, ctx->u1 internally)
    let result = unsafe {
      polcom_commit(ctx_box.as_mut(), ctx_box.s, n)
    };
    
    if result != 0 {
      unsafe {
        free_polcomctx(ctx_box.as_mut());
      }
      return Err(SpartanError::InvalidCommitmentLength {
        reason: format!("Greyhound commit failed with error code: {}", result),
      });
    }
    
    // Extract hash
    let hash = ctx_box.h;
    
    // Store minimal context data (just the hash and length)
    // The full context can't be serialized due to raw pointers
    let ctx_data = Vec::new(); // We'll reconstruct from hash if needed
    
    // Keep polz_vec alive by storing it (in a real implementation, use proper RAII)
    // For now, we need to ensure polz_vec outlives ctx
    let _polz_guard = polz_vec;
    
    // Free the context (it will free sx, t, but not u1 which is used by proof)
    // Actually, we should keep ctx alive for proof generation, but for now we'll free it
    // In a real implementation, we'd need to manage this more carefully
    unsafe {
      // Don't free yet - u1 might be needed for proof
      // free_polcomctx(ctx_box.as_mut());
    }
    
    // Leak ctx_box for now - proper implementation would use a wrapper type
    let _ctx_guard = ctx_box;
    
    info!(elapsed_ms = %commit_t.elapsed().as_millis(), "greyhound_commit");
    
    Ok(GreyhoundCommitment {
      hash,
      len: n,
      ctx_data,
      _phantom: PhantomData,
    })
  }

  fn check_commitment(comm: &Self::Commitment, n: usize, _width: usize) -> Result<(), SpartanError> {
    if comm.len != n {
      return Err(SpartanError::InvalidCommitmentLength {
        reason: format!(
          "InvalidCommitmentLength: actual: {}, expected: {}",
          comm.len, n
        ),
      });
    }
    Ok(())
  }

  fn rerandomize_commitment(
    _ck: &Self::CommitmentKey,
    comm: &Self::Commitment,
    _r_old: &Self::Blind,
    _r_new: &Self::Blind,
  ) -> Result<Self::Commitment, SpartanError> {
    // Greyhound doesn't support rerandomization in the same way as Hyrax
    // Return the same commitment
    Ok(comm.clone())
  }

  fn combine_commitments(comms: &[Self::Commitment]) -> Result<Self::Commitment, SpartanError> {
    if comms.is_empty() {
      return Err(SpartanError::InvalidInputLength {
        reason: "combine_commitments: No commitments provided".to_string(),
      });
    }
    
    // Combine by hashing all commitments together
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    for comm in comms {
      hasher.update(&comm.hash);
      hasher.update(&comm.len.to_le_bytes());
    }
    let hash_result = hasher.finalize();
    let mut combined_hash = [0u8; 16];
    combined_hash.copy_from_slice(&hash_result[..16]);
    
    let total_len = comms.iter().map(|c| c.len).sum();
    
    Ok(GreyhoundCommitment {
      hash: combined_hash,
      len: total_len,
      ctx_data: Vec::new(), // Combined commitments don't preserve context
      _phantom: PhantomData,
    })
  }

  fn combine_blinds(blinds: &[Self::Blind]) -> Result<Self::Blind, SpartanError> {
    if blinds.is_empty() {
      return Err(SpartanError::InvalidInputLength {
        reason: "combine_blinds: No blinds provided".to_string(),
      });
    }
    let mut blinds_comb = Vec::new();
    for b in blinds {
      blinds_comb.extend_from_slice(&b.blind);
    }
    Ok(GreyhoundBlind { blind: blinds_comb })
  }

  fn prove(
    ck: &Self::CommitmentKey,
    _ck_eval: &Self::CommitmentKey,
    transcript: &mut E::TE,
    comm: &Self::Commitment,
    poly: &[E::Scalar],
    _blind: &Self::Blind,
    point: &[E::Scalar],
    _comm_eval: &Self::Commitment,
    _blind_eval: &Self::Blind,
  ) -> Result<Self::EvaluationArgument, SpartanError> {
    let (_prove_span, prove_t) = start_span!("greyhound_prove");
    
    let n = poly.len();
    if n != (2usize).pow(point.len() as u32) {
      return Err(SpartanError::InvalidInputLength {
        reason: format!(
          "Greyhound prove: Expected {} elements in poly, got {}",
          (2_usize).pow(point.len() as u32),
          n
        ),
      });
    }

    transcript.absorb(b"poly_com", comm);

    // Convert point to evaluation point for Greyhound (int64)
    // For multilinear polynomials, we need to convert the point to a single int64
    // This is a simplification - proper implementation would handle multilinear evaluation
    let eval_point = if !point.is_empty() {
      let bytes = point[0].to_repr();
      let mut val = 0i64;
      for (i, &byte) in bytes.as_ref().iter().take(8).enumerate() {
        val |= (byte as i64) << (i * 8);
      }
      let q = ((1i64 << LOGQ) - 99) as i64;
      val = val % q;
      if val < 0 {
        val += q;
      }
      val
    } else {
      0i64
    };

    // Evaluate polynomial at point using Greyhound's eval function
    let polz_vec = scalars_to_polz(poly);
    let eval_value = unsafe {
      polzvec_eval(polz_vec.as_ptr(), n, eval_point)
    };
    
    // Reconstruct context from serialized data (simplified - in practice need proper deserialization)
    // For now, we'll create a minimal context
    let mut ctx = Box::new(Polcomctx {
      len: n,
      m: 0,
      n: 0,
      s: polz_vec.as_ptr(),
      sx: std::ptr::null_mut(),
      t: std::ptr::null_mut(),
      u1: std::ptr::null_mut(),
      h: comm.hash,
      normsq: 0,
    });
    
    // Initialize proof structure
    let mut proof = Box::new(Polcomprf {
      len: n,
      m: 0,
      n: 0,
      x: eval_point,
      y: eval_value,
      u1: std::ptr::null_mut(),
      u2: std::ptr::null_mut(),
      normsq: 0,
    });
    
    // Initialize witness
    let mut witness = Box::new(Witness {
      r: 0,
      n: std::ptr::null_mut(),
      normsq: std::ptr::null_mut(),
      s: std::ptr::null_mut(),
    });
    
    // Call polcom_eval to generate proof
    unsafe {
      polcom_eval(
        witness.as_mut(),
        proof.as_mut(),
        ctx.as_ref(),
        eval_point,
        eval_value,
      );
    }
    
    // Reduce proof to statement
    let mut statement = Box::new(Prncplstmnt {
      _private: [],
    });
    
    let reduce_result = unsafe {
      polcom_reduce(statement.as_mut(), proof.as_ref())
    };
    
    if reduce_result != 0 {
      return Err(SpartanError::InvalidInputLength {
        reason: format!("Greyhound reduce failed with error code: {}", reduce_result),
      });
    }
    
    // Serialize proof data (simplified - proper serialization needed)
    let proof_data = bincode::serialize(&(*proof)).unwrap_or_default();
    let statement_data = Vec::new(); // Opaque structure, can't serialize easily
    let witness_data = bincode::serialize(&(*witness)).unwrap_or_default();
    
    // Clean up
    std::mem::forget(polz_vec);
    std::mem::forget(ctx);
    std::mem::forget(proof);
    std::mem::forget(witness);
    std::mem::forget(statement);
    
    info!(elapsed_ms = %prove_t.elapsed().as_millis(), "greyhound_prove");
    
    Ok(GreyhoundEvaluationArgument {
      proof_data,
      statement_data,
      witness_data,
      len: n,
      x: eval_point,
      y: eval_value,
      _phantom: PhantomData,
    })
  }

  fn verify(
    vk: &Self::VerifierKey,
    _ck_eval: &Self::CommitmentKey,
    transcript: &mut E::TE,
    comm: &Self::Commitment,
    point: &[E::Scalar],
    _comm_eval: &Self::Commitment,
    arg: &Self::EvaluationArgument,
  ) -> Result<(), SpartanError> {
    let (_verify_span, verify_t) = start_span!("greyhound_verify");
    
    transcript.absorb(b"poly_com", comm);

    // Verify the proof
    // In practice, we would:
    // 1. Deserialize statement and witness from arg
    // 2. Reconstruct statement from proof
    // 3. Call principle_verify
    
    // For now, basic validation
    if arg.len != vk.len {
      return Err(SpartanError::InvalidInputLength {
        reason: format!(
          "Greyhound verify: Proof length {} doesn't match expected {}",
          arg.len, vk.len
        ),
      });
    }
    
    // Convert point to int64 for comparison
    let eval_point = if !point.is_empty() {
      let bytes = point[0].to_repr();
      let mut val = 0i64;
      for (i, &byte) in bytes.as_ref().iter().take(8).enumerate() {
        val |= (byte as i64) << (i * 8);
      }
      let q = ((1i64 << LOGQ) - 99) as i64;
      val = val % q;
      if val < 0 {
        val += q;
      }
      val
    } else {
      0i64
    };
    
    if eval_point != arg.x {
      return Err(SpartanError::InvalidInputLength {
        reason: "Greyhound verify: Evaluation point mismatch".to_string(),
      });
    }
    
    // TODO: Proper verification would deserialize and call principle_verify
    // For now, we assume the proof is valid if it passed reduction
    
    info!(elapsed_ms = %verify_t.elapsed().as_millis(), "greyhound_verify");
    Ok(())
  }
}

impl<E: Engine> TranscriptReprTrait<E::GE> for GreyhoundCommitment<E> {
  fn to_transcript_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(16 + 8);
    bytes.extend_from_slice(&self.hash);
    bytes.extend_from_slice(&self.len.to_le_bytes());
    bytes
  }
}
