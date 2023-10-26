use crate::curves::{g1::G1Projective, g2::G2Projective, BW6_761};
use ark_algebra_test_templates::*;
use ark_scale::ark_serialize;

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<BW6_761>; msm);
test_pairing!(pairing; super::BW6_761);
