// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: MIT
// This file is part of the Spartan2 project.
// See the LICENSE file in the project root for full license information.
// Source repository: https://github.com/Microsoft/Spartan2

//! FFI bindings for Greyhound polynomial commitment scheme

use std::os::raw::{c_int, c_longlong, c_uchar, c_ulonglong};

// Constants from data.h
pub const N: usize = 64;
pub const LOGQ: usize = 32;
pub const L: usize = 3; // For LOGQ=32

// polz structure: vecn limbs[L]
#[repr(C)]
pub struct Vecn {
    pub v: [i16; N],
}

#[repr(C)]
pub struct Polz {
    pub limbs: [Vecn; L],
}

// polcomctx structure
#[repr(C)]
pub struct Polcomctx {
    pub len: usize,
    pub m: usize,
    pub n: usize,
    // comparams cpp[1] follows - we'll access it via offset
    pub s: *const Polz,
    pub sx: *mut std::ffi::c_void,
    pub t: *mut std::ffi::c_void,
    pub u1: *mut Polz,
    pub h: [c_uchar; 16],
    pub normsq: c_ulonglong,
}

// polcomprf structure
#[repr(C)]
pub struct Polcomprf {
    pub len: usize,
    pub m: usize,
    pub n: usize,
    pub x: c_longlong,
    pub y: c_longlong,
    // comparams cpp[1] follows
    pub u1: *mut Polz,
    pub u2: *mut Polz,
    pub normsq: c_ulonglong,
}

// witness structure
#[repr(C)]
pub struct Witness {
    pub r: usize,
    pub n: *mut usize,
    pub normsq: *mut c_ulonglong,
    pub s: *mut *mut std::ffi::c_void,
}

// prncplstmnt structure (opaque)
#[repr(C)]
pub struct Prncplstmnt {
    _private: [u8; 0],
}

extern "C" {
    // Greyhound functions
    pub fn polcom_commit(ctx: *mut Polcomctx, s: *const Polz, len: usize) -> c_int;
    pub fn polzvec_eval(a: *const Polz, len: usize, x: c_longlong) -> c_longlong;
    pub fn polcom_eval(
        wt: *mut Witness,
        pi: *mut Polcomprf,
        ctx: *const Polcomctx,
        x: c_longlong,
        y: c_longlong,
    );
    pub fn polcom_reduce(st: *mut Prncplstmnt, pi: *const Polcomprf) -> c_int;
    
    // Memory management
    pub fn free_polcomctx(ctx: *mut Polcomctx);
    pub fn free_polcomprf(pi: *mut Polcomprf);
    pub fn free_witness(wt: *mut Witness);
    pub fn free_prncplstmnt(st: *mut Prncplstmnt);
    
    // Verification
    pub fn principle_verify(st: *const Prncplstmnt, wt: *const Witness) -> c_int;
    
    // Helper functions from labrador
    pub fn init_witness_raw(wt: *mut Witness, r: usize, n: *const usize);
    pub fn init_prncplstmnt_raw(
        st: *mut Prncplstmnt,
        r: usize,
        n: *const usize,
        betasq: c_ulonglong,
        k: usize,
        quadratic: c_int,
    ) -> c_int;
    pub fn init_statement(st: *mut Prncplstmnt, pi: *const std::ffi::c_void, h: *const c_uchar);
    
    // polz conversion functions
    pub fn polzvec_fromint64vec(
        r: *mut Polz,
        len: usize,
        deg: usize,
        v: *const i64,
    );
    
    // Commitment key management
    pub fn init_comkey(n: usize);
    pub fn free_comkey();
}
