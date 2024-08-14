use derive_builder::Builder;
use infernal::tableout::traits::Tabular;
use serde::{Deserialize, Serialize};

use super::{ncbi_tax::NcbiTaxId, sequence_kind::SequenceKind};

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct SpeciesEntry {
    bits: f64,
    e_value: f64,
    sequence_kind: SequenceKind,
    sequence_name: String,
    overlap: Option<String>,
    ncbi_tax_id: NcbiTaxId,
    species: String,
    extra: String,
    tax_string: Vec<String>,
}

impl Tabular for SpeciesEntry {
    fn columns() -> usize {
        9
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

    pub fn species(&self) -> &str {
        &self.species
    }

    pub fn extra(&self) -> &str {
        &self.extra
    }

    pub fn tax_string(&self) -> &[String] {
        &self.tax_string
    }

    pub fn ncbi_tax_id(&self) -> &NcbiTaxId {
        &self.ncbi_tax_id
    }
}
