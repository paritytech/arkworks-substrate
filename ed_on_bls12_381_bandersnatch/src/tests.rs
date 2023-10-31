use crate::{EdwardsProjective, SWProjective};
use ark_algebra_test_templates::*;

test_group!(te; EdwardsProjective; te);
test_group!(sw; SWProjective; sw);
