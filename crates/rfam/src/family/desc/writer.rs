use std::{
    fs::File,
    path::{Path, PathBuf},
};

use strum::IntoEnumIterator;
use textwrap::{fill, LineEnding, Options, WordSeparator, WordSplitter};
use thiserror::Error;

use super::{desc_file::DescFile, fields::Field};

#[derive(Debug, Error)]
pub enum DescWriterError {
    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),

    #[error("Failed to open file {0:?}, cause: {1}")]
    FailedOpen(PathBuf, std::io::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct DescWriter<W> {
    max_width: usize,
    writer: W,
}

impl DescWriter<File> {
    pub fn to_path(path: &Path) -> Result<Self, DescWriterError> {
        let file =
            File::create(path).map_err(|e| DescWriterError::FailedOpen(PathBuf::from(path), e))?;
        Ok(Self::to_writer(file))
    }
}

impl<W> DescWriter<W>
where
    W: std::io::Write,
{
    pub fn to_writer(writer: W) -> Self {
        Self {
            max_width: 79,
            writer,
        }
    }
}

impl<W> DescWriter<W>
where
    W: std::io::Write,
{
    fn write_field(&mut self, field: &Field, text: &str) -> Result<(), DescWriterError> {
        let prefix = format!("{:<5}", field.to_string());
        let options = Options::new(self.max_width)
            .initial_indent(&prefix)
            .subsequent_indent(&prefix)
            .line_ending(LineEnding::LF)
            .word_splitter(WordSplitter::NoHyphenation)
            .word_separator(WordSeparator::AsciiSpace);
        let wrapped = fill(text, options);
        writeln!(&mut self.writer, "{}", wrapped)?;
        Ok(())
    }

    fn write_unwrapped(&mut self, field: &Field, text: &str) -> Result<(), DescWriterError> {
        writeln!(&mut self.writer, "{:<5}{}", field.to_string(), text)?;
        Ok(())
    }

    fn write_cutoff(&mut self, field: &Field, cutoff: f64) -> Result<(), DescWriterError> {
        let text = format!("{0:.2}", cutoff);
        self.write_field(field, &text)
    }

    pub fn write(&mut self, desc: &DescFile) -> Result<(), DescWriterError> {
        for field in Field::iter() {
            match field {
                Field::Accession => self.write_field(&field, desc.accession())?,
                Field::Id => self.write_field(&field, desc.id())?,
                Field::PreviousIds => {
                    let pids = desc.previous_ids();
                    if pids.is_empty() {
                        continue;
                    }
                    let ids = pids
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join("; ");
                    self.write_field(&field, &ids)?
                }
                Field::Description => self.write_field(&field, desc.description())?,
                Field::Author => {
                    for author in desc.authors().authors() {
                        self.write_field(&field, &author.to_string())?;
                    }
                }
                Field::SecondaryStructureEvidence => {
                    self.write_field(&field, &desc.seed_source().to_string())?
                }
                Field::SecondaryStructureSource => {
                    self.write_field(&field, &desc.secondary_structure_source().to_string())?
                }
                Field::GatheringThreshold => self.write_cutoff(&field, desc.gathering_cutoff())?,
                Field::TrustedCutoff => self.write_cutoff(&field, desc.trusted_cutoff())?,
                Field::NoiseCutoff => self.write_cutoff(&field, desc.noise_cutoff())?,
                Field::RnaType => self.write_field(&field, &desc.rna_type().to_string())?,
                Field::BuildCommand => self.write_field(&field, desc.build_command())?,
                Field::CalibrateCommand => self.write_field(&field, desc.calibrate_command())?,
                Field::SearchCommand => self.write_field(&field, desc.search_command())?,
                Field::ClanId => match desc.clan_id() {
                    None => (),
                    Some(c) => self.write_field(&field, c)?,
                },
                Field::DatabaseReference => {
                    for xref in desc.external_databases().to_vec() {
                        self.write_unwrapped(&field, &xref.to_string())?;
                    }
                }
                Field::Comment => match desc.comment() {
                    None => (),
                    Some(s) => self.write_field(&field, s)?,
                },
                Field::ReferenceNumber => {
                    for reference in desc.references().to_vec() {
                        let number = format!("[{}]", reference.index());
                        self.write_field(&field, &number)?;
                        self.write_field(&Field::ReferencePmid, reference.pmid())?;
                        self.write_field(&Field::ReferenceTitle, reference.title())?;
                        self.write_field(&Field::ReferenceAuthor, reference.authors())?;
                        self.write_field(&Field::ReferenceLocation, reference.citation())?;
                    }
                }
                Field::ReferenceTitle
                | Field::ReferenceLocation
                | Field::ReferenceAuthor
                | Field::ReferencePmid => continue,
                Field::WikiArticle => self.write_field(&field, desc.wikipedia_article_name())?,
                Field::Other => {
                    for other in desc.other_fields() {
                        self.write_field(&field, other)?;
                    }
                }
            }
        }
        Ok(())
    }
}
