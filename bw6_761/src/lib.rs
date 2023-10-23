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

pub use ark_bw6_761_ext::{fq, fq::*, fq3, fq3::*, fq6, fq6::*, fr, fr::*, CurveHooks};
use ark_std::vec::Vec;
pub use curves::*;

#[derive(Copy, Clone)]
pub struct Host;

impl CurveHooks for Host {
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_multi_miller_loop(a, b)
    }
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_final_exponentiation(f12)
    }
    fn bw6_761_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_msm_g1(bases, bigints)
    }
    fn bw6_761_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_msm_g2(bases, bigints)
    }
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_mul_projective_g1(base, scalar)
    }
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_mul_projective_g2(base, scalar)
    }
}
