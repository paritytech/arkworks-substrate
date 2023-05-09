use sp_ark_bls12_381::g1::Config as ConfigHost;
use sp_ark_bls12_381::{
    curves::g1::endomorphism as endomorphismHost, G1Affine as G1AffineHost,
    G1Projective as G1ProjectiveHost,
};
use sp_ark_models::short_weierstrass::Affine;

pub type Config = ConfigHost<crate::Host>;

pub type G1Affine = G1AffineHost<crate::Host>;
pub type G1Projective = G1ProjectiveHost<crate::Host>;

pub use sp_ark_bls12_381::curves::g1::{BETA, G1_GENERATOR_X, G1_GENERATOR_Y};

pub fn endomorphism(p: &Affine<Config>) -> Affine<Config> {
    endomorphismHost::<crate::Host>(p)
}
