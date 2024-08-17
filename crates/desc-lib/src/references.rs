use std::num::NonZeroUsize;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Builder, Serialize, Deserialize)]
pub struct DescReference {
    index: NonZeroUsize,
    pmid: String,
    title: String,
    authors: String,
    citation: String,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct References {
    references: Vec<DescReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReferenceEdit {
    /// Add a new reference
    AddReference(DescReference),

    /// Remove all references
    Clear,

    /// Remove a reference with the given index.
    RemoveByIndex(NonZeroUsize),

    /// Remove a reference with the given pmid.
    RemoveByPmid(String),
}

impl DescReference {
    pub fn builder() -> DescReferenceBuilder {
        DescReferenceBuilder::default()
    }

    pub fn index(&self) -> NonZeroUsize {
        self.index
    }

    pub fn pmid(&self) -> &str {
        &self.pmid
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn citation(&self) -> &str {
        &self.citation
    }

    pub fn authors(&self) -> &str {
        &self.authors
    }
}

impl References {
    pub fn insert(&mut self, reference: DescReference) {
        // TODO Ensure it is unique
        // TODO Validate the indexes
        self.references.push(reference);
    }

    // pub(crate) fn push(&mut self, reference: DescReference) {
    //     self.references.push(reference);
    // }

    fn reindex(&mut self) {
        let mut index: NonZeroUsize = NonZeroUsize::MIN;
        for reference in &mut self.references {
            reference.index = index;
            index = index.saturating_add(1);
        }
    }

    fn remove<P>(&mut self, pred: P)
    where
        P: FnMut(&DescReference) -> bool,
    {
        let position = self.references.iter().position(pred);
        match position {
            None => (),
            Some(p) => {
                self.references.remove(p);
                self.reindex()
            }
        }
    }

    pub fn edit(&mut self, edit: ReferenceEdit) {
        match edit {
            ReferenceEdit::AddReference(desc_ref) => {
                self.insert(desc_ref);
            }
            ReferenceEdit::RemoveByPmid(pmid) => {
                self.remove(|r| r.pmid == pmid);
            }
            ReferenceEdit::RemoveByIndex(index) => {
                self.remove(|r| r.index == index);
            }
            ReferenceEdit::Clear => {
                self.references = Vec::new();
            }
        }
    }

    pub fn to_vec(&self) -> &[DescReference] {
        &self.references
    }
}
