#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code
)]
#![allow(clippy::result_unit_err)]

pub mod curves;

pub use ark_bls12_377_ext::{
    fq12, fq2, fr, Fq, Fq12Config, Fq2, Fq2Config, Fq6Config, Fr, FrConfig,
};
pub use curves::*;
