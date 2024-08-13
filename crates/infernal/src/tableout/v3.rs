use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{
    inclusion::Inclusion, model_type::ModelType, one_based_range::OneBasedRange,
    sequence_range::SequenceRange, strand::Strand, traits::Tabular, truncation::Truncation,
};

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Hit {
    target_name: String,
    target_accession: Option<String>,
    query_name: String,
    query_accession: Option<String>,
    clan_name: Option<String>,
    model_type: ModelType,
    model_range: OneBasedRange,
    sequence_range: SequenceRange,
    strand: Strand,
    truncation: Truncation,
    pass: String,
    gc: f64,
    bias: f64,
    score: f64,
    e_value: f64,
    passes_inclusion_threshold: Inclusion,
    model_length: usize,
    sequence_length: usize,
    description: Option<String>,
}

impl Tabular for Hit {
    fn columns() -> usize {
        20
    }
}

impl Hit {
    pub fn builder() -> HitBuilder {
        HitBuilder::default()
    }
}
