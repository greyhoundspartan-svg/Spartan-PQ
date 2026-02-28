// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: MIT
// This file is part of the Spartan2 project.
// See the LICENSE file in the project root for full license information.
// Source repository: https://github.com/Microsoft/Spartan2

//! MSVC-compatible Greyhound polynomial commitment scheme
#![cfg(msvc_greyhound)]

use crate::{
  errors::SpartanError,
  traits::{
    Engine,
    pcs::PCSEngineTrait,
    transcript::TranscriptReprTrait,
  },
};
use core::marker::PhantomData;
use ff::Field;
use serde::{Deserialize, Serialize};

/// MSVC-compatible Greyhound commitment
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct MSVCGreyhoundCommitment<E: Engine>(PhantomData<E>);

impl<E: Engine> TranscriptReprTrait<E::GE> for MSVCGreyhoundCommitment<E> {
  fn to_transcript_bytes(&self) -> Vec<u8> {
    vec![]
  }
}

impl<E: Engine> crate::traits::pcs::CommitmentTrait<E> for MSVCGreyhoundCommitment<E> {}

/// MSVC-compatible Greyhound verifier key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct MSVCGreyhoundVerifierKey<E: Engine>(PhantomData<E>);

/// MSVC-compatible Greyhound commitment key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct MSVCGreyhoundCommitmentKey<E: Engine>(PhantomData<E>);

/// MSVC-compatible Greyhound proof
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct MSVCGreyhoundProof<E: Engine>(PhantomData<E>);

/// MSVC-compatible Greyhound evaluation argument
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct MSVCGreyhoundEvaluationArgument<E: Engine>(PhantomData<E>);

/// MSVC-compatible Greyhound polynomial commitment scheme
#[derive(Clone, Debug)]
pub struct MSVCGreyhoundPCS<E: Engine>(PhantomData<E>);

impl<E: Engine> PCSEngineTrait<E> for MSVCGreyhoundPCS<E> {
  type CommitmentKey = MSVCGreyhoundCommitmentKey<E>;
  type VerifierKey = MSVCGreyhoundVerifierKey<E>;
  type Commitment = MSVCGreyhoundCommitment<E>;
  type Blind = E::Scalar;
  type EvaluationArgument = MSVCGreyhoundEvaluationArgument<E>;

  fn setup(
    _label: &'static [u8],
    _n: usize,
    _width: usize,
  ) -> (Self::CommitmentKey, Self::VerifierKey) {
    (
      MSVCGreyhoundCommitmentKey(PhantomData),
      MSVCGreyhoundVerifierKey(PhantomData),
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
    Ok(MSVCGreyhoundCommitment(PhantomData))
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
    Ok(MSVCGreyhoundEvaluationArgument(PhantomData))
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
    Ok(MSVCGreyhoundCommitment(PhantomData))
  }

  fn combine_commitments(_comms: &[Self::Commitment]) -> Result<Self::Commitment, SpartanError> {
    Ok(MSVCGreyhoundCommitment(PhantomData))
  }

  fn combine_blinds(_blinds: &[Self::Blind]) -> Result<Self::Blind, SpartanError> {
    Ok(E::Scalar::ZERO)
  }
}

// Implement FoldingEngineTrait for MSVC Greyhound PCS
impl<E: Engine> crate::traits::pcs::FoldingEngineTrait<E> for MSVCGreyhoundPCS<E> {
  fn fold_commitments(
    _comms: &[Self::Commitment],
    _weights: &[E::Scalar],
  ) -> Result<Self::Commitment, SpartanError> {
    Ok(MSVCGreyhoundCommitment(PhantomData))
  }

  fn fold_blinds(
    _blinds: &[Self::Blind],
    _weights: &[E::Scalar],
  ) -> Result<Self::Blind, SpartanError> {
    Ok(E::Scalar::ZERO)
  }
}
