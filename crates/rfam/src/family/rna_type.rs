/// This module represents the RNA types that Rfam knows about. Rfam has a small fixed set of RNA
/// types that are allowed. These mostly map to things in more well known systems, eg the sequence
/// ontology, but the structure and naming is specific for Rfam.
use serde::{Deserialize, Serialize};

/// All Rfam RNA types.
#[derive(Clone, Debug, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum RnaType {
    /// A fallback term for any cis-regulatory element.
    #[strum(serialize = "Cis-reg;")]
    CisReg,

    /// A term for an Internal Ribosome Entry Site.
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
