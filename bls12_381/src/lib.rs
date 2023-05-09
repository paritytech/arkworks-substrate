#![cfg_attr(not(feature = "std"), no_std)]
pub mod curves;

#[cfg(feature = "r1cs")]
pub use ark_bls12_381::constraints::*;

pub use curves::*;
pub use sp_ark_bls12_381::{
    fq, fq::*, fq12, fq12::*, fq2, fq2::*, fq6, fq6::*, fr, fr::*, HostFunctions,
};

#[derive(PartialEq, Eq)]
pub struct Host;

impl HostFunctions for Host {
    fn bls12_381_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_381_multi_miller_loop(a, b)
    }
    fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_381_final_exponentiation(f12)
    }
    fn bls12_381_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_381_msm_g1(bases, bigints)
    }
    fn bls12_381_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_381_msm_g2(bases, bigints)
    }
    fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_381_mul_projective_g1(base, scalar)
    }
    fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_381_mul_projective_g2(base, scalar)
    }
}
