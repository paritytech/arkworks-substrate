pub mod g1;
pub mod g2;
use ark_bls12_377_ext::Config as ConfigHost;

pub type Config = ConfigHost<crate::Host>;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

#[cfg(test)]
mod tests;
