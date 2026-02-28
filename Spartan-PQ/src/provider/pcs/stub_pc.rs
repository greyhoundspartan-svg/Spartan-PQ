// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: MIT
// This file is part of the Spartan2 project.
// See the LICENSE file in the project root for full license information.
// Source repository: https://github.com/Microsoft/Spartan2

//! Stub polynomial commitment scheme for when Greyhound is not available
#![cfg(stub_greyhound)]

use crate::{
  errors::SpartanError,
  traits::{
    Engine,
    pcs::{PCSEngineTrait, CommitmentTrait},
    transcript::TranscriptReprTrait,
  },
};
use core::marker::PhantomData;
use ff::Field;
use serde::{Deserialize, Serialize};

/// Stub commitment
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct StubCommitment<E: Engine>(PhantomData<E>);

impl<E: Engine> CommitmentTrait<E> for StubCommitment<E> {}

impl<E: Engine> TranscriptReprTrait<E::GE> for StubCommitment<E> {
  fn to_transcript_bytes(&self) -> Vec<u8> {
    vec![]
  }
}

/// Stub verifier key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct StubVerifierKey<E: Engine>(PhantomData<E>);

/// Stub commitment key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct StubCommitmentKey<E: Engine>(PhantomData<E>);

/// Stub proof
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct StubProof<E: Engine>(PhantomData<E>);

/// Stub evaluation argument
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct StubEvaluationArgument<E: Engine>(PhantomData<E>);

/// Stub polynomial commitment scheme
#[derive(Clone, Debug)]
pub struct StubPCS<E: Engine>(PhantomData<E>);

impl<E: Engine> PCSEngineTrait<E> for StubPCS<E> {
  type CommitmentKey = StubCommitmentKey<E>;
  type VerifierKey = StubVerifierKey<E>;
  type Commitment = StubCommitment<E>;
  type Blind = E::Scalar;
  type EvaluationArgument = StubEvaluationArgument<E>;

  fn setup(
    _label: &'static [u8],
    _n: usize,
    _width: usize,
  ) -> (Self::CommitmentKey, Self::VerifierKey) {
    (
      StubCommitmentKey(PhantomData),
      StubVerifierKey(PhantomData),
    )
  }

  fn blind(_ck: &Self::CommitmentKey, _n: usize) -> Self::Blind {
    E::Scalar::ZERO
  }

  fn commit(
    _ck: &Self::CommitmentKey,
    _v: &[E::Scalar],
    _r: &Self::Blind,
    _is_small: bool,
  ) -> Result<Self::Commitment, SpartanError> {
    // On platforms where Greyhound is not available (e.g. MSVC without GCC),
    // we use a stub PCS that does not provide binding/hiding guarantees but
    // is sufficient for exercising the higher-level Spartan logic.
    Ok(StubCommitment(PhantomData))
  }

  fn prove(
    _ck: &Self::CommitmentKey,
    _ck_eval: &Self::CommitmentKey,
    _transcript: &mut E::TE,
    _comm: &Self::Commitment,
    _poly: &[E::Scalar],
    _blind: &Self::Blind,
    _point: &[E::Scalar],
    _comm_eval: &Self::Commitment,
    _blind_eval: &Self::Blind,
  ) -> Result<Self::EvaluationArgument, SpartanError> {
    Ok(StubEvaluationArgument(PhantomData))
  }

  fn verify(
    _vk: &Self::VerifierKey,
    _ck_eval: &Self::CommitmentKey,
    _transcript: &mut E::TE,
    _comm: &Self::Commitment,
    _point: &[E::Scalar],
    _comm_eval: &Self::Commitment,
    _arg: &Self::EvaluationArgument,
  ) -> Result<(), SpartanError> {
    Ok(())
  }

  fn check_commitment(_comm: &Self::Commitment, _n: usize, _width: usize) -> Result<(), SpartanError> {
    Ok(())
  }

  fn rerandomize_commitment(
    _ck: &Self::CommitmentKey,
    _comm: &Self::Commitment,
    _r_old: &Self::Blind,
    _r_new: &Self::Blind,
  ) -> Result<Self::Commitment, SpartanError> {
    Ok(_comm.clone())
  }

  fn combine_commitments(_comms: &[Self::Commitment]) -> Result<Self::Commitment, SpartanError> {
    if let Some(first) = _comms.first() {
      Ok(first.clone())
    } else {
      Ok(StubCommitment(PhantomData))
    }
  }

  fn combine_blinds(_blinds: &[Self::Blind]) -> Result<Self::Blind, SpartanError> {
    // Simple linear combination: sum of blinds (only used in tests on this stub backend)
    let mut acc = E::Scalar::ZERO;
    for b in _blinds {
      acc += *b;
    }
    Ok(acc)
  }
}

// Implement FoldingEngineTrait for stub PCS
impl<E: Engine> crate::traits::pcs::FoldingEngineTrait<E> for StubPCS<E> {
  fn fold_commitments(
    _comms: &[Self::Commitment],
    _weights: &[E::Scalar],
  ) -> Result<Self::Commitment, SpartanError> {
    // Return a representative commitment; weights are ignored in the stub.
    if let Some(first) = _comms.first() {
      Ok(first.clone())
    } else {
      Ok(StubCommitment(PhantomData))
    }
  }

  fn fold_blinds(
    _blinds: &[Self::Blind],
    _weights: &[E::Scalar],
  ) -> Result<Self::Blind, SpartanError> {
    // Simple weighted sum of blinds for compatibility with higher-level code.
    let mut acc = E::Scalar::ZERO;
    let len = core::cmp::min(_blinds.len(), _weights.len());
    for i in 0..len {
      acc += _blinds[i] * _weights[i];
    }
    Ok(acc)
  }
}
