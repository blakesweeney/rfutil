use derive_builder::Builder;
use infernal::tableout::traits::Tabular;
use serde::{Deserialize, Serialize};

use super::sequence_kind::SequenceKind;

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ScoreEntry {
    sequence_id: String,
    start: usize,
    end: usize,
    sequence_accession: String,
    bit_score: f64,
    e_value: f64,
    model_start: usize,
    model_end: usize,
    marker: usize,
    sequence_kind: SequenceKind,
}

impl Tabular for ScoreEntry {
    fn columns() -> usize {
        9
    }
}

impl ScoreEntry {
    pub fn model_start(&self) -> usize {
        self.model_start
    }

    pub fn model_end(&self) -> usize {
        self.model_end
    }

    pub fn sequence_kind(&mut self) -> &SequenceKind {
        &self.sequence_kind
    }

    pub fn sequence_id(&self) -> &str {
        &self.sequence_id
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.sequence_accession.as_bytes()
    }

    pub fn bit_score(&self) -> f64 {
        self.bit_score
    }

    pub fn e_value(&self) -> f64 {
        self.e_value
    }

    pub fn marker(&self) -> usize {
        self.marker
    }
}
