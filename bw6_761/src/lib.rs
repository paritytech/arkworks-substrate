#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use ark_bw6_761_ext::{fq, fq::*, fq3, fq3::*, fq6, fq6::*, fr, fr::*};

pub mod curves {
    pub use sp_crypto_ec_utils::bw6_761::{
        g1, g2, Config, G1Affine, G1Config, G1Projective, G2Affine, G2Config, G2Projective, BW6_761,
    };
}

pub use curves::*;
