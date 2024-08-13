use std::num::NonZeroUsize;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    inclusion::Inclusion, model_type::ModelType, strand::Strand, traits::Tabular,
    truncation::Truncation,
};

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Hit {
    target_name: String,
    target_accession: Option<String>,
    query_name: String,
    query_accession: Option<String>,
    model_type: ModelType,
    model_from: NonZeroUsize,
    model_to: NonZeroUsize,
    seq_from: NonZeroUsize,
    seq_to: NonZeroUsize,
    strand: Strand,
    truncation: Truncation,
    pass: String,
    gc: f64,
    bias: f64,
    score: f64,
    e_value: f64,
    passes_inclusion_threshold: Inclusion,
    description: Option<String>,
}

#[derive(Debug, Error)]
pub enum BuildingError {
    #[error("Could not parse {0:?}")]
    InvalidParts(Vec<String>),
}

impl Tabular for Hit {
    fn columns() -> usize {
        18
    }
}

impl Hit {
    pub fn builder() -> HitBuilder {
        HitBuilder::default()
    }

    pub fn target_name(&self) -> &str {
        self.target_name.as_ref()
    }

    pub fn target_accession(&self) -> Option<&String> {
        self.target_accession.as_ref()
    }

    pub fn query_name(&self) -> &str {
        self.query_name.as_ref()
    }

    pub fn query_accession(&self) -> Option<&String> {
        self.query_accession.as_ref()
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

    pub fn fc(&self) -> f64 {
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

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn model_from(&self) -> NonZeroUsize {
        self.model_from
    }

    pub fn model_to(&self) -> NonZeroUsize {
        self.model_to
    }

    pub fn seq_from(&self) -> NonZeroUsize {
        self.seq_from
    }

    pub fn seq_to(&self) -> NonZeroUsize {
        self.seq_to
    }
}
