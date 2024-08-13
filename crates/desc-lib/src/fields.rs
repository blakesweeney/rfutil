use strum;

pub enum MergeAction {
    NoMerge,
    Combine,
    SpaceSeperated,
}

#[derive(Clone, Debug, PartialEq, Eq, strum::EnumString, strum::Display, strum::EnumIter)]
pub enum Field {
    #[strum(serialize = "AC")]
    Accession,
    #[strum(serialize = "ID")]
    Id,
    #[strum(serialize = "PI")]
    PreviousIds,
    #[strum(serialize = "DE")]
    Description,
    #[strum(serialize = "AU")]
    Author,
    #[strum(serialize = "SE")]
    SecondaryStructureEvidence,
    #[strum(serialize = "SS")]
    SecondaryStructureSource,
    #[strum(serialize = "GA")]
    GatheringThreshold,
    #[strum(serialize = "TC")]
    TrustedCutoff,
    #[strum(serialize = "NC")]
    NoiseCutoff,
    #[strum(serialize = "TP")]
    RnaType,
    #[strum(serialize = "BM")]
    BuildCommand,
    #[strum(serialize = "CB")]
    CalibrateCommand,
    #[strum(serialize = "SM")]
    SearchCommand,
    #[strum(serialize = "DR")]
    DatabaseReference,
    #[strum(serialize = "RN")]
    ReferenceNumber,
    #[strum(serialize = "RT")]
    ReferenceTitle,
    #[strum(serialize = "RL")]
    ReferenceLocation,
    #[strum(serialize = "RA")]
    ReferenceAuthor,
    #[strum(serialize = "RM")]
    ReferencePmid,
    #[strum(serialize = "CC")]
    Comment,
    #[strum(serialize = "WK")]
    WikiArticle,
    #[strum(serialize = "CL")]
    ClanId,
    #[strum(serialize = "**")]
    Other,
}

impl Field {
    pub fn merge_action(&self) -> MergeAction {
        match self {
            Field::Accession => MergeAction::Combine,
            Field::Id => MergeAction::Combine,
            Field::PreviousIds => MergeAction::NoMerge,
            Field::Description => MergeAction::SpaceSeperated,
            Field::SecondaryStructureEvidence => MergeAction::NoMerge,
            Field::SecondaryStructureSource => MergeAction::NoMerge,
            Field::GatheringThreshold => MergeAction::NoMerge,
            Field::TrustedCutoff => MergeAction::NoMerge,
            Field::NoiseCutoff => MergeAction::NoMerge,
            Field::RnaType => MergeAction::NoMerge,
            Field::ClanId => MergeAction::NoMerge,
            Field::DatabaseReference => MergeAction::NoMerge,
            Field::Comment => MergeAction::SpaceSeperated,
            Field::ReferenceNumber => MergeAction::NoMerge,
            Field::ReferenceTitle => MergeAction::SpaceSeperated,
            Field::ReferenceLocation => MergeAction::SpaceSeperated,
            Field::ReferenceAuthor => MergeAction::NoMerge,
            Field::ReferencePmid => MergeAction::NoMerge,
            Field::WikiArticle => MergeAction::Combine,
            Field::Author => MergeAction::NoMerge,
            Field::BuildCommand => MergeAction::SpaceSeperated,
            Field::CalibrateCommand => MergeAction::SpaceSeperated,
            Field::SearchCommand => MergeAction::SpaceSeperated,
            Field::Other => MergeAction::NoMerge,
        }
    }
}
