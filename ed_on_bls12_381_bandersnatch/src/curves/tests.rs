use crate::{EdwardsProjective, SWProjective};
use ark_algebra_test_templates::*;
use ark_scale::ark_serialize;

test_group!(te; EdwardsProjective; te);
test_group!(sw; SWProjective; sw);
