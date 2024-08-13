use serde::{Deserialize, Serialize};

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, strum::EnumString, Serialize, Deserialize,
)]
pub enum Strand {
    #[strum(serialize = "+")]
    Forward,

    #[strum(serialize = "-")]
    Reverse,
}
