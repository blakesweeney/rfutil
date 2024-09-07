use anyhow::Result;
use openapi::apis::{
    configuration::Configuration, ontology_term_controller_api::get_term, urlencode,
};
use rfam::family::desc::{
    authors::AuthorEdit,
    database_references::{DatabaseReference, XrefEdit},
    desc_file::Edit,
    references::ReferenceEdit,
};

pub fn fetch_ols_name(ontology: &str, ont_id: &str) -> Result<Option<String>> {
    let conf = Configuration::default();
    let iri = urlencode(format!(
        "http://purl.obolibrary.org/obo/{ontology}_{ont_id}"
    ));
    let term = get_term(&conf, ontology, &iri, None)?;
    return Ok(term.label);
}

pub fn add(term: &str) -> Result<Edit> {
    match term.split_once(':') {
        Some((db, id)) => {
            let db_name = db.to_uppercase();
            match db_name.as_ref() {
                "HTTP" | "HTTPS" => {
                    let db_ref = DatabaseReference::new("URL".to_string(), term.to_string(), None);
                    Ok(Edit::Xref(XrefEdit::AddOrUpdate(db_ref)))
                }
                "GO" | "SO" => {
                    let db_ref = DatabaseReference::new(
                        db_name.clone(),
                        id.to_string(),
                        fetch_ols_name(&db_name, id)?,
                    );
                    Ok(Edit::Xref(XrefEdit::AddOrUpdate(db_ref)))
                }
                _ => Err(anyhow::anyhow!("Cannot yet fetch from database {}", &db)),
            }
        }
        None => Err(anyhow::anyhow!("Cannot handle reference {:?}", term)),
    }
}

pub fn remove(term: &str) -> Result<Edit> {
    if term == "SO" || term == "GO" {
        return Ok(Edit::Xref(XrefEdit::RemoveDb(term.to_string())));
    }

    match term.split_once(':') {
        Some((db, id)) => {
            let db_name = db.to_uppercase();
            match db_name.as_ref() {
                "HTTP" | "HTTPS" => Ok(Edit::Xref(XrefEdit::RemoveEntry {
                    db: "URL".to_string(),
                    internal_id: term.to_string(),
                })),
                "GO" | "SO" => {
                    if id.is_empty() {
                        Ok(Edit::Xref(XrefEdit::RemoveDb(db_name)))
                    } else {
                        Ok(Edit::Xref(XrefEdit::RemoveEntry {
                            db: db_name,
                            internal_id: id.to_string(),
                        }))
                    }
                }
                "PMID" => Ok(Edit::Reference(ReferenceEdit::RemoveByPmid(id.to_string()))),
                "ORCID" => Ok(Edit::Author(AuthorEdit::RemoveByOrcid(id.to_string()))),
                _ => Err(anyhow::anyhow!(
                    "Cannot determine what kind of reference {} is",
                    &term
                )),
            }
        }
        None => Err(anyhow::anyhow!("Cannot handle reference {:?}", term)),
    }
}
