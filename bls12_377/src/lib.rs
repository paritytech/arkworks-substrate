#![cfg_attr(not(feature = "std"), no_std)]
pub mod curves;

pub use curves::*;
pub use sp_ark_bls12_377::{
    fq12, fq2, fr, Fq, Fq12Config, Fq2, Fq2Config, Fq6Config, Fr, FrConfig,
};
use sp_ark_bls12_377::{Bls12_377 as Bls12_377Host, HostFunctions};
use ark_std::vec::Vec;

#[derive(PartialEq, Eq)]
pub struct Host;

impl HostFunctions for Host {
    fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_377_multi_miller_loop(a, b)
    }
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_377_final_exponentiation(f12)
    }
    fn bls12_377_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_377_msm_g1(bases, bigints)
    }
    fn bls12_377_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_377_msm_g2(bases, bigints)
    }
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_377_mul_projective_g1(base, scalar)
    }
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_io::elliptic_curves::bls12_377_mul_projective_g2(base, scalar)
    }
}

pub type Bls12_377 = Bls12_377Host<Host>;
