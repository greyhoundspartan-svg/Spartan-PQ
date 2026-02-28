// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: MIT
// This file is part of the Spartan2 project.
// See the LICENSE file in the project root for full license information.
// Source repository: https://github.com/Microsoft/Spartan2

//! This module implements Spartan's traits using the following several different combinations

// public modules to be used as an commitment engine with Spartan
pub mod keccak;
pub mod pasta;
pub mod pcs;
pub mod pt256;
pub mod traits;

mod msm;

use crate::{
  provider::{
    keccak::Keccak256Transcript,
    pt256::t256,
  },
  traits::Engine,
};
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[cfg(not(stub_greyhound))]
use crate::provider::pcs::greyhound_pc::GreyhoundPCS;

#[cfg(stub_greyhound)]
use crate::provider::pcs::stub_pc::StubPCS;

#[cfg(not(stub_greyhound))]
/// An implementation of the Spartan Engine trait with Pallas curve and Greyhound commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PallasGreyhoundEngine;

#[cfg(not(stub_greyhound))]
/// An implementation of the Spartan Engine trait with Vesta curve and Greyhound commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VestaGreyhoundEngine;

#[cfg(not(stub_greyhound))]
/// An implementation of the Spartan Engine trait with P256 curve and Greyhound commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct P256GreyhoundEngine;

#[cfg(not(stub_greyhound))]
/// An implementation of the Spartan Engine trait with T256 curve and Greyhound commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct T256GreyhoundEngine;

#[cfg(not(stub_greyhound))]
impl Engine for PallasGreyhoundEngine {
  type Base = crate::provider::pasta::pallas::Base;
  type Scalar = crate::provider::pasta::pallas::Scalar;
  type GE = crate::provider::pasta::pallas::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = GreyhoundPCS<Self>;
}

#[cfg(not(stub_greyhound))]
impl Engine for VestaGreyhoundEngine {
  type Base = crate::provider::pasta::vesta::Base;
  type Scalar = crate::provider::pasta::vesta::Scalar;
  type GE = crate::provider::pasta::vesta::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = GreyhoundPCS<Self>;
}

#[cfg(not(stub_greyhound))]
impl Engine for P256GreyhoundEngine {
  type Base = crate::provider::pt256::p256::Base;
  type Scalar = crate::provider::pt256::p256::Scalar;
  type GE = crate::provider::pt256::p256::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = GreyhoundPCS<Self>;
}

#[cfg(not(stub_greyhound))]
impl Engine for T256GreyhoundEngine {
  type Base = t256::Base;
  type Scalar = t256::Scalar;
  type GE = t256::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = GreyhoundPCS<Self>;
}

// Backward compatibility aliases (deprecated - use Greyhound engines instead)
#[cfg(not(stub_greyhound))]
#[deprecated(note = "Use PallasGreyhoundEngine instead")]
pub type PallasHyraxEngine = PallasGreyhoundEngine;

#[cfg(not(stub_greyhound))]
#[deprecated(note = "Use VestaGreyhoundEngine instead")]
pub type VestaHyraxEngine = VestaGreyhoundEngine;

#[cfg(not(stub_greyhound))]
#[deprecated(note = "Use P256GreyhoundEngine instead")]
pub type P256HyraxEngine = P256GreyhoundEngine;

#[cfg(not(stub_greyhound))]
#[deprecated(note = "Use T256GreyhoundEngine instead")]
pub type T256HyraxEngine = T256GreyhoundEngine;

// Fallback engine for when Greyhound is not available (MSVC builds)
#[cfg(stub_greyhound)]
/// An implementation of the Spartan Engine trait with Pallas curve and placeholder commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PallasGreyhoundEngine;

#[cfg(stub_greyhound)]
/// An implementation of the Spartan Engine trait with Vesta curve and placeholder commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VestaGreyhoundEngine;

#[cfg(stub_greyhound)]
/// An implementation of the Spartan Engine trait with P256 curve and placeholder commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct P256GreyhoundEngine;

#[cfg(stub_greyhound)]
/// An implementation of the Spartan Engine trait with T256 curve and placeholder commitment scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct T256GreyhoundEngine;

#[cfg(stub_greyhound)]
impl Engine for PallasGreyhoundEngine {
  type Base = crate::provider::pasta::pallas::Base;
  type Scalar = crate::provider::pasta::pallas::Scalar;
  type GE = crate::provider::pasta::pallas::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = StubPCS<Self>;
}

#[cfg(stub_greyhound)]
impl Engine for VestaGreyhoundEngine {
  type Base = crate::provider::pasta::vesta::Base;
  type Scalar = crate::provider::pasta::vesta::Scalar;
  type GE = crate::provider::pasta::vesta::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = StubPCS<Self>;
}

#[cfg(stub_greyhound)]
impl Engine for P256GreyhoundEngine {
  type Base = crate::provider::pt256::p256::Base;
  type Scalar = crate::provider::pt256::p256::Scalar;
  type GE = crate::provider::pt256::p256::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = StubPCS<Self>;
}

#[cfg(stub_greyhound)]
impl Engine for T256GreyhoundEngine {
  type Base = t256::Base;
  type Scalar = t256::Scalar;
  type GE = t256::Point;
  type TE = Keccak256Transcript<Self>;
  type PCS = StubPCS<Self>;
}