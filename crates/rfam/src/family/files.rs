/// This module covers the types of files that can be part of, or produced during the curation of,
/// an Rfam family.

/// There are two kinds of Rfam families, `PreSeed`, and `Modern`. A PreSeed family is missing
/// two files, relative to a `Modern` family. Generally all families should be migrated to a
/// 'modern' setup now, however when working with older versions, eg any family pre 2024, they may
/// be preseed and thus missing files.
pub enum FamilyKind {
    /// A 'pre-seed' family, these families are missing the `SEEDTBLOUT` and `SEEDSCORES` files.
    PreSeed,

    /// A 'modern' family, which will include a `SEEDTBLOUT` and a `SEEDSCORES` files.
    Modern,
}

/// These are the files which are always part of an Rfam family.
#[derive(Clone, Debug, PartialEq, strum::EnumString, strum::Display)]
pub enum CoreFiles {
    /// The SEED file, which contains the alignment
    #[strum(serialize = "SEED")]
    Seed,

    /// The DESC file which contains all metadata related to a family.
    #[strum(serialize = "DESC")]
    Desc,

    /// The covariance model.
    #[strum(serialize = "CM")]
    Cm,

    /// The TBLOUT file for the reversed matches only.
    #[strum(serialize = "REVTBLOUT")]
    RevTblout,

    /// The SCORES file which covers the score information for all matches sequences
    #[strum(serialize = "SCORES")]
    Scores,

    /// The TBLOUT file which has the hits for all the family in Rfamseq. This is a tblout file from
    /// infernal's cmsearch.
    #[strum(serialize = "TBLOUT")]
    Tblout,
}

/// These files are files which are part of modern families and not the `PreSeed` families.
#[derive(Clone, Debug, PartialEq, strum::EnumString, strum::Display)]
pub enum ModernOnly {
    /// A TBLOUT file for the SEED seequences only.
    #[strum(serialize = "SEEDTBLOUT")]
    SeedTblout,

    /// The SEEDSCORES file, which has scores for the SEED hits only.
    #[strum(serialize = "SEEDSCORES")]
    SeedScores,
}
