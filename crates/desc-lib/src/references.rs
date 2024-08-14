use std::num::NonZeroUsize;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::edit::ReferenceEdit;

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
        self.references.push(reference);
    }

    pub fn edit(&mut self, edit: ReferenceEdit) {
        match edit {
            ReferenceEdit::AddReference(_) => todo!(),
            ReferenceEdit::RemoveByPmid(_) => todo!(),
            ReferenceEdit::RemoveByIndex(_) => todo!(),
        }
    }

    pub fn to_vec(&self) -> &[DescReference] {
        &self.references
    }
}
