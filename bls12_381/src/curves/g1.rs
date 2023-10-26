use crate::HostHooks;

pub use ark_bls12_381_ext::g1::{BETA, G1_GENERATOR_X, G1_GENERATOR_Y};

pub type Config = ark_bls12_381_ext::g1::Config<HostHooks>;

pub type G1Affine = ark_bls12_381_ext::g1::G1Affine<HostHooks>;
pub type G1Projective = ark_bls12_381_ext::g1::G1Projective<HostHooks>;

#[inline(always)]
pub fn endomorphism(p: &G1Affine) -> G1Affine {
    ark_bls12_381_ext::g1::endomorphism(p)
}
