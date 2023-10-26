use crate::HostHooks;

pub use ark_bw6_761_ext::g2::{G2_GENERATOR_X, G2_GENERATOR_Y};

pub type G2Affine = ark_bw6_761_ext::g2::G2Affine<HostHooks>;
pub type G2Projective = ark_bw6_761_ext::g2::G2Projective<HostHooks>;

pub type Config = ark_bw6_761_ext::g2::Config<HostHooks>;
