use sp_ark_bw6_761::curves::g1::{
    Config as ConfigHost, G1Affine as G1AffineHost, G1Projective as G1ProjectiveHost,
};

pub type G1Affine = G1AffineHost<crate::Host>;
pub type G1Projective = G1ProjectiveHost<crate::Host>;

pub type Config = ConfigHost<crate::Host>;

pub use sp_ark_bw6_761::curves::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
