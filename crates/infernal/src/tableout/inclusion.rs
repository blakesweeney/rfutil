use serde::{Deserialize, Serialize};

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, strum::EnumString, Serialize, Deserialize,
)]
pub enum Inclusion {
    #[strum(serialize = "!")]
    Included,

    #[strum(serialize = "?")]
    ReportingOnly,
}
