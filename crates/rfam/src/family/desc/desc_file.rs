use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::family::rna_type::RnaType;

use super::{
    authors::{Author, AuthorEdit, Authors},
    database_references::{DatabaseReference, DatabaseReferences, XrefEdit},
    references::{DescReference, ReferenceEdit, References},
    secondary_structure::SecondaryStructureSource,
    seed_evidence::SeedEvidence,
};

#[derive(Debug, Error)]
pub enum DescEditError {}

#[derive(Clone, PartialEq, Debug, Builder)]
pub struct DescFile {
    accession: String,
    id: String,
    #[builder(default)]
    previous_ids: Vec<String>,
    description: String,
    #[builder(default)]
    authors: Authors,
    seed_source: SeedEvidence,
    secondary_structure_source: SecondaryStructureSource,
    gathering_cutoff: f64,
    trusted_cutoff: f64,
    noise_cutoff: f64,
    rna_type: RnaType,
    build_command: String,
    search_command: String,
    calibrate_command: String,
    #[builder(setter(strip_option), default)]
    clan_id: Option<String>,
    #[builder(default)]
    external_databases: DatabaseReferences,
    #[builder(default)]
    references: References,
    #[builder(setter(strip_option), default)]
    comment: Option<String>,
    wikipedia_article_name: String,
    #[builder(default)]
    other_fields: Vec<String>,
}

/// Changes that can be made to the clan setting.
#[derive(Debug, Serialize, Deserialize)]
pub enum ClanEdit {
    /// Remove the clan.
    Clear,
    /// Set the clan to the given clan id
    Set(String),
}

/// Changes that can be made to a comment field.
#[derive(Debug, Serialize, Deserialize)]
pub enum CommentChange {
    /// Remove the comment string
    Clear,

    /// Set the comment string to the given text
    Set(String),
}

/// The edits that can be made to a `DescFile`.
///
/// There are some limits to changing DescFiles. Notably this cannot change the id or accession of a
/// desc file. The id has to changed by using an rfam script `rfmove.pl`, while the accession cannot
/// be changed and is handled by the Rfam curation pipeline. Thus these fields are considered
/// immutable for this code.
#[derive(Debug, Serialize, Deserialize)]
pub enum Edit {
    /// Set the description to the given text
    Description(String),

    /// Set the gathering threshold to the given value.
    GatheringThreshold(f64),

    /// Edits to the author setting
    Author(AuthorEdit),

    /// Change the RNA type to the given one
    RnaType(RnaType),

    /// Edits to clans
    Clan(ClanEdit),

    /// Edits to the database references
    Xref(XrefEdit),

    /// Edits to the reference literature
    Reference(ReferenceEdit),

    /// Update the wikipedia article to the given one
    WikiArticle(String),

    /// Edits to the comments.
    Comment(CommentChange),
}

impl DescFileBuilder {
    pub fn add_xref(&mut self, xref: DatabaseReference) -> &mut Self {
        let xs = self
            .external_databases
            .get_or_insert_with(DatabaseReferences::default);
        xs.insert(xref);
        self
    }

    pub fn add_reference(&mut self, reference: DescReference) -> &mut Self {
        let refs = self.references.get_or_insert_with(References::default);
        refs.insert(reference);
        self
    }

    pub fn add_author(&mut self, author: Author) -> &mut Self {
        let authors = self.authors.get_or_insert_with(Authors::default);
        authors.insert(author);
        self
    }

    pub fn add_other(&mut self, other: String) -> &mut Self {
        let others = self.other_fields.get_or_insert_with(Vec::default);
        others.push(other);
        self
    }
}

impl DescFile {
    /// Create a new `DescFileBuilder`.
    ///
    /// ## Example
    /// ```
    /// let builder = DescFile::builder();
    /// ...
    /// let desc_file = builder.build()?;
    /// ````
    pub fn builder() -> DescFileBuilder {
        DescFileBuilder::default()
    }

    // Edit this `DescFile`.
    //
    // ## Example
    //
    // ```
    // let edit = Edit::WikiArticle("ncRna");
    // desc.edit(edit)?;
    // assert_eq!(desc.wikiarticle(), "ncRna");
    // ```
    pub fn edit(&mut self, edit: Edit) -> Result<(), DescEditError> {
        match edit {
            Edit::Description(d) => {
                self.description = d;
            }
            Edit::GatheringThreshold(t) => {
                self.gathering_cutoff = t;
            }
            Edit::RnaType(rna_type) => {
                self.rna_type = rna_type;
            }
            Edit::WikiArticle(name) => {
                self.wikipedia_article_name = name;
            }
            Edit::Author(cmd) => {
                self.authors.edit(cmd);
            }
            Edit::Clan(cmd) => match cmd {
                ClanEdit::Clear => {
                    self.clan_id = None;
                }
                ClanEdit::Set(c) => {
                    self.clan_id = Some(c);
                }
            },
            Edit::Xref(cmd) => self.external_databases.edit(cmd),
            Edit::Reference(cmd) => self.references.edit(cmd),
            Edit::Comment(cmd) => match cmd {
                CommentChange::Clear => {
                    self.comment = None;
                }
                CommentChange::Set(c) => {
                    self.comment = Some(c);
                }
            },
        }
        Ok(())
    }

    pub fn accession(&self) -> &str {
        self.accession.as_ref()
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn secondary_structure_source(&self) -> &SecondaryStructureSource {
        &self.secondary_structure_source
    }

    pub fn gathering_cutoff(&self) -> f64 {
        self.gathering_cutoff
    }

    pub fn trusted_cutoff(&self) -> f64 {
        self.trusted_cutoff
    }

    pub fn noise_cutoff(&self) -> f64 {
        self.noise_cutoff
    }

    pub fn clan_id(&self) -> Option<&String> {
        self.clan_id.as_ref()
    }

    pub fn external_databases(&self) -> &DatabaseReferences {
        &self.external_databases
    }

    pub fn authors(&self) -> &Authors {
        &self.authors
    }

    pub fn seed_source(&self) -> &SeedEvidence {
        &self.seed_source
    }

    pub fn rna_type(&self) -> &RnaType {
        &self.rna_type
    }

    pub fn build_command(&self) -> &str {
        &self.build_command
    }

    pub fn search_command(&self) -> &str {
        &self.search_command
    }

    pub fn calibrate_command(&self) -> &str {
        &self.calibrate_command
    }

    pub fn references(&self) -> &References {
        &self.references
    }

    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    pub fn wikipedia_article_name(&self) -> &str {
        &self.wikipedia_article_name
    }

    pub fn other_fields(&self) -> &[String] {
        &self.other_fields
    }

    pub fn previous_ids(&self) -> &Vec<String> {
        &self.previous_ids
    }
}
