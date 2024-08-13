use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::fragment::Fragment;

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Alignment {
    non_cannonical: String,
    secondary_structure: String,
    model_consensus: Fragment,
    scoring: String,
    target_sequence: Fragment,
    probability: String,
}
