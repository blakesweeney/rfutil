use serde::{Deserialize, Serialize};

use crate::{
    authors::AuthorEdit, database_references::XrefEdit, references::ReferenceEdit,
    rna_type::RnaType,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClanEdit {
    Clear,
    Set(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommentChange {
    Clear,
    Set(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Edit {
    Description(String),
    GatheringThreshold(f64),
    Author(AuthorEdit),
    RnaType(RnaType),
    Clan(ClanEdit),
    Xref(XrefEdit),
    Reference(ReferenceEdit),
    WikiArticle(String),
    Comment(CommentChange),
}
