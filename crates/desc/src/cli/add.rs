use anyhow::Result;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};

use desc_lib::{
    database_references::DatabaseReference,
    edit::{Edit, XrefEdit},
};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoInfo {
    id: String,
    is_obsolete: bool,
    name: Option<String>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    number_of_hits: usize,
    results: Vec<T>,
}

pub fn fetch_go_name(go_id: &str) -> Result<Option<String>> {
    let url = format!(
        "https://www.ebi.ac.uk/QuickGO/services/ontology/go/terms/{}",
        &go_id
    );
    let response: Response<GoInfo> = get(url)?.json()?;
    match response.results.first() {
        None => Err(anyhow::anyhow!("No results for {}", &go_id)),
        Some(r) => Ok(r.name.clone()),
    }
}

pub fn reference_edit(term: &str) -> Result<Edit> {
    match term.split_once(":") {
        Some((db, id)) => {
            let db_name = db.to_uppercase();
            match db_name.as_ref() {
                "HTTP" | "HTTPS" => {
                    let db_ref = DatabaseReference::new("URL".to_string(), term.to_string(), None);
                    Ok(Edit::Xref(XrefEdit::AddOrUpdate(db_ref)))
                }
                "GO" => {
                    let db_ref =
                        DatabaseReference::new(db_name, id.to_string(), fetch_go_name(term)?);
                    Ok(Edit::Xref(XrefEdit::AddOrUpdate(db_ref)))
                }
                _ => Err(anyhow::anyhow!("Cannot yet fetch from database {}", &db)),
            }
        }
        None => Err(anyhow::anyhow!("Cannot handle reference {:?}", term)),
    }
}
