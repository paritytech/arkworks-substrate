#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use ark_bls12_377_ext::{fq12, fq12::*, fq2, fq2::*, fr, fr::*, Fq, Fq6Config};

pub mod curves {
    pub use sp_crypto_ec_utils::bls12_377::{
        g1, g2, Bls12_377, Config, G1Affine, G1Config, G1Projective, G2Affine, G2Config,
        G2Projective,
    };
}

pub use curves::*;
