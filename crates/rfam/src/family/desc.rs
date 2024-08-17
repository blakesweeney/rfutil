use std::{fs::File, path::Path};

use desc_file::DescFile;
use reader::DescParser;
use writer::DescWriter;

pub mod authors;
pub mod database_references;
pub mod desc_file;
pub mod fields;
pub mod reader;
pub mod references;
pub mod secondary_structure;
pub mod seed_evidence;
pub mod writer;

pub fn from_path(path: &Path) -> Result<DescFile, reader::DescParserError> {
    let reader = DescParser::from_path(path)?;
    reader.parse()
}

pub fn from_file(file: File) -> Result<DescFile, reader::DescParserError> {
    let reader = DescParser::from_reader(file);
    reader.parse()
}

pub fn to_path(path: &Path, desc: &DescFile) -> Result<(), writer::DescWriterError> {
    let mut writer = DescWriter::to_path(path)?;
    writer.write(desc)?;
    Ok(())
}

pub fn to_file(file: File, desc: &DescFile) -> Result<(), writer::DescWriterError> {
    let mut writer = DescWriter::to_writer(file);
    writer.write(desc)?;
    Ok(())
}

pub fn to_text(desc: &DescFile) -> Result<String, writer::DescWriterError> {
    let mut buffer = Vec::new();
    let mut writer = DescWriter::to_writer(&mut buffer);
    writer.write(desc)?;
    Ok(String::from_utf8(buffer).unwrap())
}
