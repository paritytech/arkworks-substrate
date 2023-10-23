use ark_bls12_377_ext::g1::{
    Config as ConfigHost, G1Affine as G1AffineHost, G1Projective as G1ProjectiveHost,
    G1SWAffine as G1SWAffineHost, G1TEAffine as G1TEAffineHost,
    G1TEProjective as G1TEProjectiveHost,
};

pub type Config = ConfigHost<crate::Host>;

pub type G1Affine = G1AffineHost<crate::Host>;
pub type G1Projective = G1ProjectiveHost<crate::Host>;

pub type G1SWAffine = G1SWAffineHost<crate::Host>;
pub type G1TEAffine = G1TEAffineHost<crate::Host>;
pub type G1TEProjective = G1TEProjectiveHost<crate::Host>;

pub use ark_bls12_377_ext::g1::{G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y};
