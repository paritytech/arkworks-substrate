use crate::HostHooks;

pub use ark_bw6_761_ext::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};

pub type G1Affine = ark_bw6_761_ext::g1::G1Affine<HostHooks>;
pub type G1Projective = ark_bw6_761_ext::g1::G1Projective<HostHooks>;

pub type Config = ark_bw6_761_ext::g1::Config<HostHooks>;
