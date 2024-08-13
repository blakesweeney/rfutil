use thiserror::Error;

use crate::{add, cli::ReplaceCommand, remove};

#[derive(Debug, Error)]
pub enum ReplaceError {
    #[error(transparent)]
    AddError(add::AddError),

    #[error(transparent)]
    RemoveError(remove::RemoveError),
}

#[derive(Subcommand, Debug)]
pub enum ReplaceCommand {
    Id {
        /// The new id to use.
        id: String,
    },

    // Description {
    //     /// The new description. If nothing is given it is prompted for.
    //     description: Vec<String>,
    // },
    RnaType {
        /// The new RNA type.
        rna_type: String,
    },

    Seed {
        source: String,
    },

    SecondaryStructure {
        method: String,
    },

    Reference {
        /// The PMID to remove
        old_pmid: String,

        /// The PMID to add.
        new_pmid: String,
    },

    OntologyTerm {
        old_term: String,
        new_term: String,
    },

    Wikipedia {
        article: String,
    },
}

impl ReplaceCommand {
    pub fn apply(&self, desc: DescFile) -> Result<DescFile, ReplaceError> {
        match self {
            ReplaceCommand::Id => Ok(Desc { id, ..desc }),
        }
    }
}

pub fn perform(command: &ReplaceCommand, desc: DescFile) -> Result<DescFile, ReplaceError> {
    todo!();
}
