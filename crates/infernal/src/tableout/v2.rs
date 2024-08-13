use std::num::NonZeroUsize;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{
    inclusion::Inclusion, model_type::ModelType, overlap_status::OverlapStatus, traits::Tabular,
    truncation::Truncation,
};

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Hit {
    index: NonZeroUsize,
    target_name: String,
    target_accession: Option<String>,
    query_name: String,
    query_accession: Option<String>,
    clan_name: Option<String>,
    model_type: ModelType,
    model_from: NonZeroUsize,
    model_to: NonZeroUsize,
    seq_from: NonZeroUsize,
    seq_to: NonZeroUsize,
    truncation: Truncation,
    pass: String,
    gc: f64,
    bias: f64,
    score: f64,
    e_value: f64,
    passes_inclusion_threshold: Inclusion,
    overlap_status: OverlapStatus,
    lowest_evalue_overlap_index: Option<NonZeroUsize>,
    best_scoring_overlap_fraction: Option<f64>,
    best_scoring_overlap_fraction_reverse: Option<f64>,
    winidx: Option<String>,
    winfrct1: Option<String>,
    winfrct2: Option<String>,
    model_length: usize,
    sequence_length: usize,
    description: Option<String>,
}

impl Tabular for Hit {
    fn columns() -> usize {
        29
    }
}

impl Hit {
    pub fn builder() -> HitBuilder {
        HitBuilder::default()
    }

    pub fn index(&self) -> NonZeroUsize {
        self.index
    }

    pub fn query_name(&self) -> &str {
        self.query_name.as_ref()
    }

    pub fn query_accession(&self) -> Option<&String> {
        self.query_accession.as_ref()
    }

    pub fn clan_name(&self) -> Option<&String> {
        self.clan_name.as_ref()
    }

    pub fn model_type(&self) -> &ModelType {
        &self.model_type
    }

    pub fn truncation(&self) -> &Truncation {
        &self.truncation
    }

    pub fn pass(&self) -> &str {
        self.pass.as_ref()
    }

    pub fn gc_content(&self) -> f64 {
        self.gc
    }

    pub fn bias(&self) -> f64 {
        self.bias
    }

    pub fn bit_score(&self) -> f64 {
        self.score
    }

    pub fn e_value(&self) -> f64 {
        self.e_value
    }

    pub fn passes_inclusion_threshold(&self) -> &Inclusion {
        &self.passes_inclusion_threshold
    }

    pub fn overlap_status(&self) -> &OverlapStatus {
        &self.overlap_status
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
