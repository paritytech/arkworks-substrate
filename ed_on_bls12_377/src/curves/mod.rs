use sp_ark_ed_on_bls12_377::{
    EdwardsAffine as EdwardsAffineHost, EdwardsConfig as EdwardsConfigHost,
    EdwardsProjective as EdwardsProjectiveHost,
};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = EdwardsAffineHost<crate::Host>;
pub type EdwardsProjective = EdwardsProjectiveHost<crate::Host>;

pub type EdwardsConfig = EdwardsConfigHost<crate::Host>;
