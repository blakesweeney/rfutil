use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct OverlapStats {
    index: NonZeroUsize,
    fraction_forward_overlaps: f64,
    fraction_reverse_overlap: f64,
}
