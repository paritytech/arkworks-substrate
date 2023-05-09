#![cfg_attr(not(feature = "std"), no_std)]
use sp_ark_ed_on_bls12_377::HostFunctions;
pub mod curves;

#[cfg(feature = "r1cs")]
pub use sp_ark_ed_on_bls12_377::constraints::*;

use ark_std::vec::Vec;
pub use curves::*;
pub use sp_ark_ed_on_bls12_377::{fq, fq::*, fr, fr::*};

pub struct Host {}

impl HostFunctions for Host {
    fn ed_on_bls12_377_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
    }
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
    }
}
