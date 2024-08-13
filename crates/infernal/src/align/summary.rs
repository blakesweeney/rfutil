use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::tableout::{model_type::ModelType, truncation::Truncation};

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, Deserialize)]
pub struct Summary {
    rank: usize,
    e_value: f64,
    score: f64,
    bias: f64,
    modelname: String,
    start: usize,
    stop: usize,
    model: ModelType,
    trunc: Truncation,
    gc: f64,
    description: Option<String>,
}
