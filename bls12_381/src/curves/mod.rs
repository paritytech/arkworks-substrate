use sp_ark_bls12_381::{Bls12_381 as Bls12_381Host, Config as ConfigHost};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub type Config = ConfigHost<crate::Host>;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

pub type Bls12_381 = Bls12_381Host<crate::Host>;
