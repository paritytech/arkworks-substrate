use ark_bls12_377_ext::CurveHooks;
use ark_ec::{pairing::Pairing, CurveConfig};
use ark_scale::{
    ark_serialize::{Compress, Validate},
    scale::{Decode, Encode},
};
use sp_crypto_ec_utils::elliptic_curves as bls12_377_ops;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{Config as G1Config, G1Affine, G1Projective},
    g2::{Config as G2Config, G2Affine, G2Projective},
};

const SCALE_USAGE: u8 = ark_scale::make_usage(Compress::No, Validate::No);
type ArkScale<T> = ark_scale::ArkScale<T, SCALE_USAGE>;
type ArkScaleProjective<T> = ark_scale::hazmat::ArkScaleProjective<T>;

#[derive(Copy, Clone)]
pub struct HostHooks;

pub type Config = ark_bls12_377_ext::Config<HostHooks>;
pub type Bls12_377 = ark_bls12_377_ext::Bls12_377<HostHooks>;

impl CurveHooks for HostHooks {
    fn bls12_377_multi_miller_loop(
        g1: impl Iterator<Item = <Bls12_377 as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <Bls12_377 as Pairing>::G2Prepared>,
    ) -> Result<<Bls12_377 as Pairing>::TargetField, ()> {
        let g1 = ArkScale::from(g1.collect::<Vec<_>>()).encode();
        let g2 = ArkScale::from(g2.collect::<Vec<_>>()).encode();

        let res = bls12_377_ops::bls12_377_multi_miller_loop(g1, g2).unwrap_or_default();
        let res = ArkScale::<<Bls12_377 as Pairing>::TargetField>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_final_exponentiation(
        target: <Bls12_377 as Pairing>::TargetField,
    ) -> Result<<Bls12_377 as Pairing>::TargetField, ()> {
        let target = ArkScale::from(target).encode();

        let res = bls12_377_ops::bls12_377_final_exponentiation(target).unwrap_or_default();
        let res = ArkScale::<<Bls12_377 as Pairing>::TargetField>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_msm_g1(
        bases: &[G1Affine],
        scalars: &[<G1Config as CurveConfig>::ScalarField],
    ) -> Result<G1Projective, ()> {
        let bases = ArkScale::from(bases).encode();
        let scalars = ArkScale::from(scalars).encode();

        let res = bls12_377_ops::bls12_377_msm_g1(bases, scalars).unwrap_or_default();
        let res = ArkScaleProjective::<G1Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_msm_g2(
        bases: &[G2Affine],
        scalars: &[<G2Config as CurveConfig>::ScalarField],
    ) -> Result<G2Projective, ()> {
        let bases = ArkScale::from(bases).encode();
        let scalars = ArkScale::from(scalars).encode();

        let res = bls12_377_ops::bls12_377_msm_g2(bases, scalars).unwrap_or_default();
        let res = ArkScaleProjective::<G2Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_mul_projective_g1(
        base: &G1Projective,
        scalar: &[u64],
    ) -> Result<G1Projective, ()> {
        let base = ArkScaleProjective::from(base).encode();
        let scalar = ArkScale::from(scalar).encode();

        let res = bls12_377_ops::bls12_377_mul_projective_g1(base, scalar).unwrap_or_default();
        let res = ArkScaleProjective::<G1Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_mul_projective_g2(
        base: &G2Projective,
        scalar: &[u64],
    ) -> Result<G2Projective, ()> {
        let base = ArkScaleProjective::from(base).encode();
        let scalar = ArkScale::from(scalar).encode();

        let res = bls12_377_ops::bls12_377_mul_projective_g2(base, scalar).unwrap_or_default();
        let res = ArkScaleProjective::<G2Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }
}
