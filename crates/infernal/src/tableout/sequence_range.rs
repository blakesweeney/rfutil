use super::one_based_range::OneBasedRange;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum SequenceRange {
    Forward(OneBasedRange),
    Reverse(OneBasedRange),
}
