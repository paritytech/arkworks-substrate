use sp_ark_models::bls12;

pub type G2Affine = bls12::G2Affine<crate::curves::Config>;
pub type G2Projective = bls12::G2Projective<crate::curves::Config>;
