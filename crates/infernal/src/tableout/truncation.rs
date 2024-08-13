use serde::{Deserialize, Serialize};

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, strum::EnumString, Serialize, Deserialize,
)]
pub enum Truncation {
    #[strum(serialize = "no")]
    No,

    #[strum(serialize = "5'")]
    FivePrime,

    #[strum(serialize = "3'")]
    ThreePrime,

    #[strum(serialize = "5'&3'")]
    Both,
}
