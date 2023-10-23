#![cfg_attr(not(feature = "std"), no_std)]

use ark_ed_on_bls12_377_ext::CurveHooks;
pub mod curves;

#[cfg(feature = "r1cs")]
pub use ark_ed_on_bls12_377_ext::constraints;

pub use ark_ed_on_bls12_377_ext::{fq, fq::*, fr, fr::*};
use ark_std::vec::Vec;
pub use curves::*;

#[derive(Copy, Clone)]
pub struct Host;

impl CurveHooks for Host {
    fn ed_on_bls12_377_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
    }
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
    }
}
