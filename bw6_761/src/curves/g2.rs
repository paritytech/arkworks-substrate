use sp_ark_bw6_761::curves::g2::{
    Config as ConfigHost, G2Affine as G2AffineHost, G2Projective as G2ProjectiveHost,
};

pub type G2Affine = G2AffineHost<crate::Host>;
pub type G2Projective = G2ProjectiveHost<crate::Host>;

pub type Config = ConfigHost<crate::Host>;

pub use sp_ark_bw6_761::curves::g2::{G2_GENERATOR_X, G2_GENERATOR_Y};
