#[derive(Clone, PartialEq, Debug)]
pub enum SeedEvidence {
    Published(String),
    Predicted(String),
    Other(String),
}

impl ToString for SeedEvidence {
    fn to_string(&self) -> String {
        match self {
            SeedEvidence::Published(s) => format!("Published; {}", s),
            SeedEvidence::Predicted(s) => format!("Predicted; {}", s),
            SeedEvidence::Other(s) => s.to_string(),
        }
    }
}
