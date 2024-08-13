use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Fragment {
    start: usize,
    stop: usize,
    text: String,
}
