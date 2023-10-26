use crate::HostHooks;

pub use ark_bls12_377_ext::g1::{G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y};

pub type Config = ark_bls12_377_ext::g1::Config<HostHooks>;

pub type G1Affine = ark_bls12_377_ext::g1::G1Affine<HostHooks>;
pub type G1Projective = ark_bls12_377_ext::g1::G1Projective<HostHooks>;

pub type G1SWAffine = ark_bls12_377_ext::g1::G1SWAffine<HostHooks>;
pub type G1SWProjective = ark_bls12_377_ext::g1::G1SWProjective<HostHooks>;
pub type G1TEAffine = ark_bls12_377_ext::g1::G1TEAffine<HostHooks>;
pub type G1TEProjective = ark_bls12_377_ext::g1::G1TEProjective<HostHooks>;
