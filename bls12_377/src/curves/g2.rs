use sp_ark_bls12_377::g2::Config as ConfigHost;
use sp_ark_models::bls12;

pub type G2Affine = bls12::G2Affine<crate::curves::Config>;
pub type G2Projective = bls12::G2Projective<crate::curves::Config>;

pub type Config = ConfigHost<crate::Host>;

pub use sp_ark_bls12_377::g2::{
    G2_GENERATOR_X, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_Y, G2_GENERATOR_Y_C0,
    G2_GENERATOR_Y_C1,
};
