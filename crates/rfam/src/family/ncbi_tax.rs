use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcbiTaxId(String);

impl From<String> for NcbiTaxId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<usize> for NcbiTaxId {
    fn from(value: usize) -> Self {
        Self(value.to_string())
    }
}
