use crate::HostHooks;

pub use ark_bls12_377_ext::g2::{
    G2_GENERATOR_X, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_Y, G2_GENERATOR_Y_C0,
    G2_GENERATOR_Y_C1,
};

pub type Config = ark_bls12_377_ext::g2::Config<HostHooks>;

pub type G2Affine = ark_bls12_377_ext::g2::G2Affine<HostHooks>;
pub type G2Projective = ark_bls12_377_ext::g2::G2Projective<HostHooks>;
