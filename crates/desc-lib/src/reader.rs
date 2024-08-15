use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::num::NonZeroUsize;
use std::path::Path;
use std::str::FromStr;

use chardet::{charset2encoding, detect};
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use thiserror::Error;

use crate::authors::Author;
use crate::database_references::DatabaseReference;
use crate::fields::Field;
use crate::fields::MergeAction;
use crate::references::DescReference;
use crate::rna_type::RnaType;
use crate::secondary_structure::SecondaryStructureSource;
use crate::seed_evidence::SeedEvidence;

use super::DescFile;

#[derive(Debug, Error)]
pub enum DescParserError {
    #[error("Separator in line `{0}` is invalid")]
    InvalidSeperator(String),

    #[error("Cannot parse author line `{0}`")]
    InvalidAuthorValue(String),

    #[error("Cannot parse database reference line `{0}`")]
    InvalidDbReference(String),

    #[error("Reference index {0} is invalid, must be a number >0")]
    InvalidIndex(String),

    #[error("Within reference block and saw unexpected field `{0:?}`")]
    UnexpectedField(Field),

    #[error("Unknown type of field {0}")]
    UnknownFieldType(String),

    #[error("Invalid {0} value: `{1}`, cause: {2}")]
    InvalidThreshold(String, String, std::num::ParseFloatError),

    #[error("Invalid RNA Type {0}")]
    InvalidRnaType(String),

    #[error("Reference line {0}, outside of reference block")]
    UnexpectedReferenceLine(String),

    #[error("Detected encoding `{0}` is unknown.")]
    UnknownEncoding(String),

    #[error(transparent)]
    DescFileBuilderError(#[from] crate::desc::DescFileBuilderError),

    #[error(transparent)]
    DescReferenceBuilderError(#[from] crate::references::DescReferenceBuilderError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

fn parse_reference(
    raw_number: &str,
    entries: &mut Vec<(Field, String)>,
) -> Result<DescReference, DescParserError> {
    let mut builder = DescReference::builder();
    let raw_index = raw_number
        .trim_matches(&['[', ']'])
        .parse()
        .map_err(|_| DescParserError::InvalidIndex(raw_number.to_string()))?;
    let index = NonZeroUsize::new(raw_index)
        .ok_or_else(|| DescParserError::InvalidIndex(raw_number.to_string()))?;
    builder.index(index);
    while let Some((field, value)) = entries.last() {
        match field {
            Field::ReferenceTitle => {
                builder.title(value.replace("  ", " ").to_string());
                entries.pop();
            }
            Field::ReferenceLocation => {
                builder.citation(value.to_string());
                entries.pop();
            }
            Field::ReferenceAuthor => {
                builder.authors(value.to_string());
                entries.pop();
            }
            Field::ReferencePmid => {
                builder.pmid(value.to_string());
                entries.pop();
            }
            _ => break,
        }
    }
    let result = builder.build();
    Ok(result?)
}

pub struct DescParser<R> {
    reader: R,
}

impl DescParser<File> {
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        Ok(Self::from_reader(file))
    }
}

impl<R> DescParser<R>
where
    R: Read,
{
    pub fn from_reader(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> DescParser<R>
where
    R: Read,
{
    pub fn parse(self) -> Result<DescFile, DescParserError> {
        let buf = {
            let mut b = Vec::new();
            let mut reader = BufReader::new(self.reader);
            reader.read_to_end(&mut b)?;
            b
        };

        let encoding = {
            let inferred = detect(&buf);
            let encoding_label = charset2encoding(&inferred.0);
            Encoding::for_label(encoding_label.as_bytes())
        };

        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(encoding)
            .build(buf.as_slice());
        let reader = BufReader::new(decoder);
        let mut entries: Vec<(Field, String)> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let cleaned = line.trim_end();
            if line.is_empty() {
                continue;
            }
            let field = Field::from_str(&cleaned[0..2])
                .map_err(|_| DescParserError::UnknownFieldType(cleaned.to_string()))?;
            let value = cleaned[5..].trim();
            if cleaned[2..5] != *"   " {
                return Err(DescParserError::InvalidSeperator(cleaned.to_string()));
            }
            let last = entries.last_mut();
            match last {
                Some((f, v)) if f == &field => match f.merge_action() {
                    MergeAction::NoMerge => {
                        entries.push((field, value.to_string()));
                    }
                    MergeAction::Combine => {
                        v.push_str(value);
                    }
                    MergeAction::SpaceSeperated => {
                        v.push(' ');
                        v.push_str(value);
                    }
                },
                _ => entries.push((field, value.to_string())),
            }
        }
        entries.reverse();

        let mut builder = DescFile::builder();
        while let Some((field, value)) = entries.pop() {
            match field {
                Field::Accession => {
                    builder.accession(value);
                }
                Field::Id => {
                    builder.id(value);
                }
                Field::PreviousIds => {
                    let ids: Vec<String> = value.split("; ").map(|s| s.to_string()).collect();
                    builder.previous_ids(ids);
                }
                Field::Description => {
                    builder.description(value);
                }
                Field::Author => {
                    let parts: Vec<&str> = value.split("; ").collect();
                    let author = match parts[..] {
                        [name] => Author::new(name.to_string(), None),
                        [name, orcid] => Author::new(name.to_string(), Some(orcid.to_string())),
                        _ => {
                            return Err(DescParserError::InvalidAuthorValue(value.clone()));
                        }
                    };
                    builder.add_author(author);
                }
                Field::SecondaryStructureEvidence => {
                    let seed = match value.split_once("; ") {
                        Some(("Predicted", rest)) => SeedEvidence::Predicted(rest.to_string()),
                        Some(("Published", rest)) => SeedEvidence::Published(rest.to_string()),
                        _ => SeedEvidence::Other(value.to_string()),
                    };
                    builder.seed_source(seed);
                }
                Field::SecondaryStructureSource => {
                    let source = SecondaryStructureSource::Other(value.to_string());
                    builder.secondary_structure_source(source);
                }
                Field::GatheringThreshold => {
                    let threshold = value.parse().map_err(|e| {
                        DescParserError::InvalidThreshold("GA".to_string(), value.to_string(), e)
                    })?;
                    builder.gathering_cutoff(threshold);
                }
                Field::TrustedCutoff => {
                    let threshold = value.parse().map_err(|e| {
                        DescParserError::InvalidThreshold("TC".to_string(), value.to_string(), e)
                    })?;
                    builder.trusted_cutoff(threshold);
                }
                Field::NoiseCutoff => {
                    let threshold = value.parse().map_err(|e| {
                        DescParserError::InvalidThreshold("NC".to_string(), value.to_string(), e)
                    })?;
                    builder.noise_cutoff(threshold);
                }
                Field::RnaType => {
                    let rna_type = RnaType::from_str(&value)
                        .map_err(|_| DescParserError::InvalidRnaType(value.to_string()))?;
                    builder.rna_type(rna_type);
                }
                Field::ClanId => {
                    builder.clan_id(value);
                }
                Field::DatabaseReference => {
                    let parts: Vec<&str> = value.split("; ").collect();
                    let xref = match parts[..] {
                        [db, id] => {
                            Ok(DatabaseReference::new(db.to_string(), id.to_string(), None))
                        }
                        [db, id, name] => Ok(DatabaseReference::new(
                            db.to_string(),
                            id.to_string(),
                            Some(name.to_string()),
                        )),
                        _ => Err(DescParserError::InvalidDbReference(value.to_string())),
                    }?;
                    builder.add_xref(xref);
                }
                Field::Comment => {
                    builder.comment(value);
                }
                Field::WikiArticle => {
                    builder.wikipedia_article_name(value);
                }
                Field::ReferenceNumber => {
                    let reference = parse_reference(&value, &mut entries)?;
                    builder.add_reference(reference);
                }
                Field::ReferenceTitle
                | Field::ReferenceLocation
                | Field::ReferenceAuthor
                | Field::ReferencePmid => {
                    return Err(DescParserError::UnexpectedReferenceLine(field.to_string()));
                }
                Field::BuildCommand => {
                    builder.build_command(value);
                }
                Field::CalibrateCommand => {
                    builder.calibrate_command(value);
                }
                Field::SearchCommand => {
                    builder.search_command(value);
                }
                Field::Other => {
                    builder.add_other(value);
                }
            }
        }

        let result = builder.build();
        Ok(result?)
    }
}
