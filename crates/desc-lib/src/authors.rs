use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::edit::AuthorEdit;

#[derive(Debug, Error)]
pub enum AuthorError {
    #[error("Duplicate author {0:?}")]
    DuplicateAuthor(Author),

    #[error("Duplicate author ORCID {0}")]
    DuplicateAuthorOrcid(String),
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Hash, Builder, Serialize, Deserialize)]
pub struct Author {
    name: String,
    orcid: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Authors {
    // TODO: Ensure authors are unique by ORCID
    authors: Vec<Author>,
}

impl Author {
    pub fn new(name: String, orcid: Option<String>) -> Self {
        Self { name, orcid }
    }
}

impl Authors {
    pub fn insert(&mut self, author: Author) {
        // TODO Ensure they are unique
        self.authors.push(author);
    }

    pub fn edit(&mut self, edit: AuthorEdit) {
        match edit {
            AuthorEdit::AddAuthor(a) => {
                self.insert(a);
            }
            AuthorEdit::RemoveByName(n) => {
                self.authors.retain(|f| f.name != n);
            }
            AuthorEdit::RemoveByOrcid(o) => self.authors.retain(|a| a.orcid != Some(o.clone())),
            AuthorEdit::SetOrcid(n, orcid) => {
                for author in self.authors.iter_mut() {
                    if author.name == n {
                        author.orcid = Some(orcid.clone());
                    }
                }
            }
        }
    }

    pub fn authors(&self) -> &Vec<Author> {
        &self.authors
    }
}

impl ToString for Author {
    fn to_string(&self) -> String {
        match &self.orcid {
            Some(o) => format!("{}; {}", self.name, o),
            None => self.name.to_string(),
        }
    }
}
