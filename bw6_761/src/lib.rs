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

pub use ark_bw6_761_ext::{fq, fq::*, fq3, fq3::*, fq6, fq6::*, fr, fr::*};
pub use curves::*;
