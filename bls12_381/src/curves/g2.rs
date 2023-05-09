use sp_ark_bls12_381::curves::g2::{
    Config as ConfigHost, G2Affine as G2AffineHost, G2Projective as G2ProjectiveHost,
};

pub type G2Affine = G2AffineHost<crate::Host>;
pub type G2Projective = G2ProjectiveHost<crate::Host>;

pub type Config = ConfigHost<crate::Host>;

pub use sp_ark_bls12_381::g2::{
    G2_GENERATOR_X, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_Y, G2_GENERATOR_Y_C0,
    G2_GENERATOR_Y_C1,
};
