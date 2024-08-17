use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum RnaType {
    #[strum(serialize = "Cis-reg;")]
    CisReg,

    #[strum(serialize = "Cis-reg; IRES;")]
    IRES,

    #[strum(serialize = "Cis-reg; frameshift_element;")]
    FrameshiftElement,

    #[strum(serialize = "Cis-reg; leader;")]
    LeaderElement,

    #[strum(serialize = "Cis-reg; riboswitch;")]
    Riboswitch,

    #[strum(serialize = "Cis-reg; thermoregulator;")]
    Thermoregulator,

    #[strum(serialize = "Gene;")]
    Gene,

    #[strum(serialize = "Gene; lncRNA;")]
    LncRnaDomain,

    #[strum(serialize = "Gene; CRISPR;")]
    CRISPR,

    #[strum(serialize = "Gene; antisense;")]
    Antisense,

    #[strum(serialize = "Gene; antitoxin;")]
    AntiToxin,

    #[strum(serialize = "Gene; miRNA;")]
    MiRNA,

    #[strum(serialize = "Gene; rRNA;")]
    RRNA,

    #[strum(serialize = "Gene; ribozyme;")]
    Ribozyme,

    #[strum(serialize = "Gene; sRNA;")]
    SRNA,

    #[strum(serialize = "Gene; snRNA;")]
    SnRNA,

    #[strum(serialize = "Gene; snRNA; snoRNA;")]
    SnoRNA,

    #[strum(serialize = "Gene; snRNA; snoRNA; CD-box;")]
    CDBoxSnoRNA,

    #[strum(serialize = "Gene; snRNA; snoRNA; HACA-box;")]
    HACABoxSnoRNA,

    #[strum(serialize = "Gene; snRNA; snoRNA; scaRNA;")]
    ScaRNA,

    #[strum(serialize = "Gene; snRNA; splicing;")]
    SplicingElement,

    #[strum(serialize = "Gene; tRNA;")]
    TRNA,

    #[strum(serialize = "Intron;")]
    Intron,
}
