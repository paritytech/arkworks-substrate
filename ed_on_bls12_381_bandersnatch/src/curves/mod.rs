use sp_ark_ed_on_bls12_381_bandersnatch::{
    BandersnatchConfig as BandersnatchConfigHost, EdwardsAffine as EdwardsAffineHost,
    EdwardsConfig as EdwardsConfigHost, EdwardsProjective as EdwardsProjectiveHost,
    SWAffine as SWAffineHost, SWConfig as SWConfigHost, SWProjective as SWProjectiveHost,
};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = EdwardsAffineHost<crate::Host>;
pub type EdwardsProjective = EdwardsProjectiveHost<crate::Host>;

pub type SWAffine = SWAffineHost<crate::Host>;
pub type SWProjective = SWProjectiveHost<crate::Host>;

pub type SWConfig = SWConfigHost<crate::Host>;
pub type EdwardsConfig = EdwardsConfigHost<crate::Host>;
pub type BandersnatchConfig = BandersnatchConfigHost<crate::Host>;
