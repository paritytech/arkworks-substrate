use ark_ff::{Field, MontFp, PrimeField, Zero};
use ark_scale::hazmat::ArkScaleProjective;
use ark_std::{marker::PhantomData, ops::Neg, One};
use codec::{Decode, Encode};
use sp_ark_models::{
    bls12,
    bls12::Bls12Config,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AffineRepr, CurveConfig, Group,
};

use crate::util::{
    read_g1_compressed, read_g1_uncompressed, serialize_fq, EncodingFlags, G1_SERIALIZED_SIZE,
};
use crate::{ArkScale, HostFunctions};
use sp_ark_bls12_381::{fr::Fr, Fq, G1Affine as G1AffineHost, G1Projective as G1ProjectiveHost};

pub type G1Affine = bls12::G1Affine<crate::Config<H>>;
pub type G1Projective = bls12::G1Projective<crate::Config<H>>;