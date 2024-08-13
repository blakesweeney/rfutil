use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct OneBasedRange {
    start: NonZeroUsize,
    stop: NonZeroUsize,
}
