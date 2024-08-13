use serde::{Deserialize, Serialize};

use crate::edit::XrefEdit;

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct DatabaseReference {
    database_name: String,
    internal_id: String,
    name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DatabaseReferences {
    xrefs: Vec<DatabaseReference>,
}

impl DatabaseReference {
    pub fn new(database_name: String, internal_id: String, name: Option<String>) -> Self {
        Self {
            database_name,
            internal_id,
            name,
        }
    }
}

impl ToString for DatabaseReference {
    fn to_string(&self) -> String {
        match &self.name {
            Some(n) => format!("{}; {}; {}", self.database_name, self.internal_id, n),
            None => format!("{}; {}", self.database_name, self.internal_id),
        }
    }
}

impl DatabaseReferences {
    pub fn insert(&mut self, xref: DatabaseReference) {
        // TODO Ensure they are unique
        self.xrefs.push(xref);
    }

    pub fn edit(&mut self, edit: XrefEdit) {
        match edit {
            XrefEdit::Add(xref) => {
                self.insert(xref);
            }
            XrefEdit::Clear => {
                self.xrefs = Vec::new();
            }
            XrefEdit::RemoveDb(db) => {
                self.xrefs.retain(|x| x.database_name != db.clone());
            }
            XrefEdit::RemoveEntry { db, internal_id } => {
                self.xrefs
                    .retain(|x| x.database_name != db && x.internal_id == internal_id);
            }
        }
    }

    pub fn to_vec(&self) -> &[DatabaseReference] {
        &self.xrefs
    }
}
