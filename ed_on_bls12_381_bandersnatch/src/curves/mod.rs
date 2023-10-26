use ark_ec::CurveConfig;
use ark_ed_on_bls12_381_bandersnatch_ext::CurveHooks;
use ark_scale::{
    ark_serialize::{Compress, Validate},
    scale::{Decode, Encode},
};
use sp_crypto_ec_utils::elliptic_curves as host_ops;

#[cfg(test)]
mod tests;

pub type BandersnatchConfig = ark_ed_on_bls12_381_bandersnatch_ext::BandersnatchConfig<HostHooks>;

pub type EdwardsConfig = ark_ed_on_bls12_381_bandersnatch_ext::EdwardsConfig<HostHooks>;
pub type EdwardsAffine = ark_ed_on_bls12_381_bandersnatch_ext::EdwardsAffine<HostHooks>;
pub type EdwardsProjective = ark_ed_on_bls12_381_bandersnatch_ext::EdwardsProjective<HostHooks>;

pub type SWConfig = ark_ed_on_bls12_381_bandersnatch_ext::SWConfig<HostHooks>;
pub type SWAffine = ark_ed_on_bls12_381_bandersnatch_ext::SWAffine<HostHooks>;
pub type SWProjective = ark_ed_on_bls12_381_bandersnatch_ext::SWProjective<HostHooks>;

const SCALE_USAGE: u8 = ark_scale::make_usage(Compress::No, Validate::No);
type ArkScale<T> = ark_scale::ArkScale<T, SCALE_USAGE>;
type ArkScaleProjective<T> = ark_scale::hazmat::ArkScaleProjective<T>;

#[derive(Copy, Clone)]
pub struct HostHooks;

impl CurveHooks for HostHooks {
    fn ed_on_bls12_381_bandersnatch_te_msm(
        bases: &[EdwardsAffine],
        scalars: &[<EdwardsConfig as CurveConfig>::ScalarField],
    ) -> Result<EdwardsProjective, ()> {
        let bases = ArkScale::from(bases).encode();
        let scalars = ArkScale::from(scalars).encode();

        let res = host_ops::ed_on_bls12_381_bandersnatch_te_msm(bases, scalars).unwrap_or_default();
        let res = ArkScaleProjective::<EdwardsProjective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn ed_on_bls12_381_bandersnatch_te_mul_projective(
        base: &EdwardsProjective,
        scalar: &[u64],
    ) -> Result<EdwardsProjective, ()> {
        let base = ArkScaleProjective::from(base).encode();
        let scalar = ArkScale::from(scalar).encode();

        let res = host_ops::ed_on_bls12_381_bandersnatch_te_mul_projective(base, scalar)
            .unwrap_or_default();
        let res = ArkScaleProjective::<EdwardsProjective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn ed_on_bls12_381_bandersnatch_sw_msm(
        bases: &[SWAffine],
        scalars: &[<SWConfig as CurveConfig>::ScalarField],
    ) -> Result<SWProjective, ()> {
        let bases = ArkScale::from(bases).encode();
        let scalars = ArkScale::from(scalars).encode();

        let res = host_ops::ed_on_bls12_381_bandersnatch_sw_msm(bases, scalars).unwrap_or_default();
        let res = ArkScaleProjective::<SWProjective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
        base: &SWProjective,
        scalar: &[u64],
    ) -> Result<SWProjective, ()> {
        let base = ArkScaleProjective::from(base).encode();
        let scalar = ArkScale::from(scalar).encode();

        let res = host_ops::ed_on_bls12_381_bandersnatch_sw_mul_projective(base, scalar)
            .unwrap_or_default();
        let res = ArkScaleProjective::<SWProjective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }
}
