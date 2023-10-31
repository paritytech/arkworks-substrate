#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use ark_ed_on_bls12_381_bandersnatch_ext::{fq, fq::*, fr, fr::*};

pub mod curves {
    pub use sp_crypto_ec_utils::ed_on_bls12_381_bandersnatch::{
        BandersnatchConfig, EdwardsAffine, EdwardsConfig, EdwardsProjective, SWAffine, SWConfig,
        SWProjective,
    };
}

pub use curves::*;

#[cfg(feature = "r1cs")]
pub use ark_ed_on_bls12_381_bandersnatch_ext::constraints;
