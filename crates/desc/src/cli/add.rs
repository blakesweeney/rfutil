use clap::Subcommand;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddError {

}

#[derive(Subcommand, Debug)]
pub enum AddCommand {
    ///
    Reference {
        /// The PMID to fetch and add to the file. If this PMID is already present this is an
        /// error and the file will not be modified.
        pmid: String,
    },

    Author {
        /// The author ORCID to add
        orcid: String,
    },

    OntologyTerm {
        /// This will add an ontology term to the DESC file. The term must be of the from:
        /// '<DB>:<id>', eg 'SO:0000370'. Currently, this supports any ontology that can be fetched
        /// from EBI's OLS: https://www.ebi.ac.uk/ols4.
        term: String,
    },
}

impl AddCommand {
    pub fn apply(&self, desc: &DescFile) -> Result<DescFile, AddError> {
        match self {
            AddCommand::Reference { pmid } => self.add_pmid(desc, &pmid),
            AddCommand::Author { orcid } => self.add_author(desc, &orcid),
            AddCommand::OntologyTerm { term } => self.add_ontology_term(desc, &term),
        }
    }
}

pub fn perform(command: &AddCommand, desc: DescFile) -> Result<DescFile, AddError> {
    Ok(())
}
