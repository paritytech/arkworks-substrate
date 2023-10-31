#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use ark_ed_on_bls12_377_ext::{fq, fq::*, fr, fr::*};

pub mod curves {
    pub use sp_crypto_ec_utils::ed_on_bls12_377::{
        EdwardsAffine, EdwardsConfig, EdwardsProjective,
    };
}

pub use curves::*;

#[cfg(feature = "r1cs")]
pub use ark_ed_on_bls12_377_ext::constraints;
