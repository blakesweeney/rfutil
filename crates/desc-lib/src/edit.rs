use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use crate::{
    authors::Author, database_references::DatabaseReference, references::DescReference,
    rna_type::RnaType,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum XrefEdit {
    AddOrUpdate(DatabaseReference),
    Clear,
    RemoveDb(String),
    RemoveEntry { db: String, internal_id: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClanEdit {
    Clear,
    Set(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthorEdit {
    AddAuthor(Author),
    RemoveByName(String),
    RemoveByOrcid(String),
    SetOrcid(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReferenceEdit {
    AddReference(DescReference),
    RemoveByPmid(String),
    RemoveByIndex(NonZeroUsize),
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
