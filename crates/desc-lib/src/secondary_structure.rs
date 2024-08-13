#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum SecondaryStructureSource {
    Published {
        pmid: String,
        authors: Vec<String>,
    },
    Predicted {
        method: String,
        authors: Vec<String>,
    },
    Other(String),
}

impl ToString for SecondaryStructureSource {
    fn to_string(&self) -> String {
        match self {
            SecondaryStructureSource::Published { pmid, authors: _ } => {
                format!("Published; {}", &pmid)
            }
            SecondaryStructureSource::Predicted { method, authors: _ } => {
                format!("Predicted; {}", &method)
            }
            SecondaryStructureSource::Other(other) => other.to_string(),
        }
    }
}
