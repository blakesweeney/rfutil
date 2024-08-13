use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{hit_alignment::HitAlignment, summary::Summary};

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Alignments {
    query: String,
    query_length: usize,
    description: Option<String>,
    summary: Vec<Summary>,
    alignments: Vec<HitAlignment>,
}
