use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub enum XrefEdit {
    AddOrUpdate(DatabaseReference),
    Clear,
    RemoveDb(String),
    RemoveEntry { db: String, internal_id: String },
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
            Some(n) => format!("{}; {}; {};", self.database_name, self.internal_id, n),
            None => format!("{}; {};", self.database_name, self.internal_id),
        }
    }
}

impl DatabaseReferences {
    pub fn insert(&mut self, xref: DatabaseReference) {
        let mut seen = false;
        for cur in &mut self.xrefs {
            if cur.database_name == xref.database_name && cur.internal_id == xref.internal_id {
                if cur.name.is_none() {
                    cur.name = xref.name.clone();
                }
                seen = true;
            }
        }
        if !seen {
            self.xrefs.push(xref)
        }
    }

    pub fn update(&mut self, xref: DatabaseReference) {
        let mut seen = false;
        for cur in &mut self.xrefs {
            if cur.database_name == xref.database_name && cur.internal_id == xref.internal_id {
                cur.name = xref.name.clone();
                seen = true;
            }
        }
        if !seen {
            self.xrefs.push(xref)
        }
    }

    pub fn edit(&mut self, edit: XrefEdit) {
        match edit {
            XrefEdit::AddOrUpdate(xref) => {
                self.update(xref);
            }
            XrefEdit::Clear => {
                self.xrefs = Vec::new();
            }
            XrefEdit::RemoveDb(db) => {
                self.xrefs.retain(|x| x.database_name != db.clone());
            }
            XrefEdit::RemoveEntry { db, internal_id } => {
                self.xrefs
                    .retain(|x| !(x.database_name == db && x.internal_id == internal_id));
            }
        }
    }

    pub fn to_vec(&self) -> &[DatabaseReference] {
        &self.xrefs
    }
}
