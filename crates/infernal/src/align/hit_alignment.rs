use derive_builder::Builder;
use serde::Serialize;

use crate::tableout::{model_type::ModelType, truncation::Truncation};

use super::alignment::Alignment;

#[derive(Clone, PartialEq, PartialOrd, Debug, Builder, Serialize, serde::Deserialize)]
pub struct HitAlignment {
    model_name: String,
    model_description: Option<String>,
    rank: usize,
    e_value: f64,
    score: f64,
    bias: f64,
    model: ModelType,
    model_from: usize,
    model_to: usize,
    sequence_from: usize,
    sequence_to: usize,
    acc: f64,
    truncation: Truncation,
    gc: f64,
    alignment: Alignment,
}
