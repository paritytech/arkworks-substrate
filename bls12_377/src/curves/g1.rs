use sp_ark_bls12_377::g1::Config as ConfigHost;
use sp_ark_models::{
    bls12,
    short_weierstrass::Affine as SWAffine,
    twisted_edwards::{Affine as TEAffine, Projective as TEProjective},
};

pub type Config = ConfigHost<crate::Host>;

pub type G1Affine = bls12::G1Affine<crate::curves::Config>;
pub type G1Projective = bls12::G1Projective<crate::curves::Config>;

pub type G1SWAffine = SWAffine<Config>;
pub type G1TEAffine = TEAffine<Config>;
pub type G1TEProjective = TEProjective<Config>;

pub use sp_ark_bls12_377::g1::{G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y};
