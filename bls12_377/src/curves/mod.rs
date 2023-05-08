pub mod g1;
pub mod g2;
use sp_ark_bls12_377::Config as ConfigHost;

pub type Config = ConfigHost<crate::Host>;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};
