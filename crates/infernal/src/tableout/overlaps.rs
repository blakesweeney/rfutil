use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, strum::EnumString, Deserialize)]
pub enum Overlaps {
    #[strum(serialize = "*")]
    NoOverlaps,
    #[strum(serialize = "^")]
    LowestEvalue,
    #[strum(serialize = "$")]
    BestScoring,
    #[strum(serialize = "=")]
    Overlaps,
}
