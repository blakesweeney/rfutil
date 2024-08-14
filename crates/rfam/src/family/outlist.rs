use derive_builder::Builder;
use infernal::tableout::{strand::Strand, traits::Tabular, truncation::Truncation};
use serde::{Deserialize, Serialize};

use super::sequence_kind::SequenceKind;

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct SpeciesEntry {
    bits: f64,
    e_value: f64,
    sequence_kind: SequenceKind,
    sequence_name: String,
    overlap: Option<String>,
    start: usize,
    end: usize,
    strand: Strand,
    model_start: usize,
    model_stop: usize,
    truncation: Truncation,
    species: String,
    extra: String,
    description: String,
}

impl Tabular for SpeciesEntry {
    fn columns() -> usize {
        14
    }
}

impl SpeciesEntry {
    pub fn bits(&self) -> f64 {
        self.bits
    }

    pub fn e_value(&self) -> f64 {
        self.e_value
    }

    pub fn sequence_kind(&self) -> &SequenceKind {
        &self.sequence_kind
    }

    pub fn sequence_name(&self) -> &str {
        &self.sequence_name
    }

    pub fn overlap(&self) -> Option<&String> {
        self.overlap.as_ref()
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn strand(&self) -> &Strand {
        &self.strand
    }

    pub fn model_start(&self) -> usize {
        self.model_start
    }

    pub fn model_stop(&self) -> usize {
        self.model_stop
    }

    pub fn truncation(&self) -> &Truncation {
        &self.truncation
    }

    pub fn species(&self) -> &str {
        &self.species
    }

    pub fn extra(&self) -> &str {
        &self.extra
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
