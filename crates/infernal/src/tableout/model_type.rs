use serde::{Deserialize, Serialize};

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, strum::EnumString, Serialize, Deserialize,
)]
pub enum ModelType {
    #[strum(serialize = "hmm")]
    Hmm,

    #[strum(serialize = "cm")]
    Cm,
}
