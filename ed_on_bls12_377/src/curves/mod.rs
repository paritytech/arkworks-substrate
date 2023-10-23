use ark_ed_on_bls12_377_ext::{
    EdwardsAffine as EdwardsAffineHost, EdwardsConfig as EdwardsConfigHost,
    EdwardsProjective as EdwardsProjectiveHost,
};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = EdwardsAffineHost<crate::Host>;
pub type EdwardsProjective = EdwardsProjectiveHost<crate::Host>;

pub type EdwardsConfig = EdwardsConfigHost<crate::Host>;
