use anyhow::Result;

use clap::Subcommand;
use desc_lib::desc::DescFile;

#[derive(Subcommand, Debug)]
pub enum RemoveCommand {
    /// Remove a reference with the given PMID.
    Reference {
        /// The PMID to remove
        pmid: String,
    },

    /// Remove an author with the given ORCID from the DESC file.
    Author {
        /// The ORCID to remove.
        orcid: String,
    },

    /// Remove a reference to an ontology term in the DESC file. This is basically the same as the
    /// external database command but with a hopefully more convient interface.
    OntologyTerm {
        /// The term to remove. This should be something like GO:000001
        term: String,
    },

    /// Remove a reference to an external database
    ExternalDatabase {
        /// The name of the database to remove from, eg GO.
        db: String,

        /// The id within the database to remove, eg 00001.
        db_id: Option<String>,
    },
}

pub fn perform(command: &RemoveCommand, desc: DescFile) -> Result<()> {
    Ok(())
}
