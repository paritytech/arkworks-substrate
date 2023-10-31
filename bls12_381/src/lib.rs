#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use ark_bls12_381_ext::{fq, fq::*, fq12, fq12::*, fq2, fq2::*, fq6, fq6::*, fr, fr::*};

pub mod curves {
    pub use sp_crypto_ec_utils::bls12_381::{
        g1, g2, Bls12_381, Config, G1Affine, G1Config, G1Projective, G2Affine, G2Config,
        G2Projective,
    };
}

pub use curves::*;

#[cfg(feature = "r1cs")]
pub use ark_bls12_381::constraints;
