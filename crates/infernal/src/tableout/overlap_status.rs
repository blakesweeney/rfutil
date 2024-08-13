use serde::{Deserialize, Serialize};

use super::overlap_stats::OverlapStats;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum OverlapStatus {
    NoOverlaps,
    LowestEvalue,
    BestScoring {
        evalue_stats: Option<OverlapStats>,
    },
    Overlaps {
        evalue_stats: OverlapStats,
        best_scoring_stats: Option<OverlapStats>,
    },
}
