/// This module contains the information related to parsing, writing and representing a DESC file in
/// Rfam. These files are responsible for storing the metadata about a family, e.g. the threshold,
/// links to wikipedia, a curated description and so forth. Generally these files are a fixed with
/// format with the first few columns dedicated to defining the line type. This module is meant
/// to make it easy to work with DESC files. An example file is below:
///
/// ```
/// AC   RF04310
/// ID   nqrA-II
/// DE   nqrA-II ncRNA motif
/// AU   Narunsky A; 0000-0002-6830-5690
/// AU   Breaker RR; 0000-0002-2165-536X
/// AU   Ontiveros-Palacios N; 0000-0001-8457-4455
/// SE   Published;  PMID:38647067
/// SS   Published;  PMID:38647067
/// GA   60.00
/// TC   68.30
/// NC   58.80
/// TP   Cis-reg;
/// BM   cmbuild -F CM SEED
/// CB   cmcalibrate --mpi CM
/// SM   cmsearch --cpu 4 --verbose --nohmmonly -T 30.00 -Z 2958934 CM SEQDB
/// DR   SO; 0000370; small_regulatory_ncRNA;
/// DR   GO; 0010468; regulation of gene expression;
/// RN   [1]
/// RM   38647067
/// RT   The discovery of novel noncoding RNAs in 50 bacterial genomes.
/// RA   Narunsky A, Higgs GA, Torres BM, Yu D, de Andrade GB, Kavita K, Breaker RR
/// RL   Nucleic Acids Res. 2024
/// CC   nqrA-II ncRNA motif was reported as a Strong Riboswitch Candidate [1]
/// CC   nqrA-II ncRNA motif was identified in five different genera of the
/// CC   Pasteurellaceae family. nqrA-II ncRNA motif was found upstream of the nqrA
/// CC   open reading frame whose protein product is annotated as a
/// CC   sodium-translocating NADH:quinone oxidoreductase (sub-unit A)
/// WK   Cis-regulatory_element
/// ```
///
/// The DESC files is represented as a `desc_file::DescFile` struct. The struct has fields for all
/// the currently used fields in DESC files. There may be more that are not covered.
///
/// One important caveat with DESC files, is that historically they were in all sorts of encodings.
/// The readers and writers try to respect that, but it is tricky and it is possible that using this
/// will cause some encoding issues. Additionally, the format was reverse engineered so it is
/// possible this does not properly handle all possible cases.
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

/// Load a DESC file at a given path.
///
/// ## Example
///
/// ```
/// let path = PathBuf::new("families/RF0001/DESC");
/// let desc = from_path(&path)?;
/// ```
pub fn from_path<P>(path: P) -> Result<DescFile, reader::DescParserError>
where
    P: AsRef<Path>,
{
    let reader = DescParser::from_path(path)?;
    reader.parse()
}

/// Load a DESC file from the given file.
///
/// ## Example
///
/// ```
/// let file = File::open("families/RF00001/DESC");
/// let desc = from_reader(file)?;
/// ```
pub fn from_reader(file: File) -> Result<DescFile, reader::DescParserError> {
    let reader = DescParser::from_reader(file);
    reader.parse()
}

/// Write the `DescFile` to the given path.
///
/// ## Example
///
/// ```
/// let path = PathBuf::new("families/RF00001/DESC");
/// to_path(&path, &desc)?;
/// ```
pub fn to_path<P>(path: P, desc: &DescFile) -> Result<(), writer::DescWriterError>
where
    P: AsRef<Path>,
{
    let mut writer = DescWriter::to_path(path)?;
    writer.write(desc)?;
    Ok(())
}

/// Write the `DescFile` to the given writer.
///
/// ## Example
///
/// ```
/// let file = File::create("families/RF0001/DESC")?;
/// to_writer(file, &desc_file)?;
/// ```
pub fn to_writer<W>(writer: W, desc: &DescFile) -> Result<(), writer::DescWriterError>
where
    W: std::io::Write,
{
    let mut writer = DescWriter::to_writer(writer);
    writer.write(desc)?;
    Ok(())
}

/// Write the `DescFile` to a string.
///
/// ## Example
///
/// ```
/// let text = to_text(&desc_file)?;
/// println!("{}", text);
/// ```
pub fn to_text(desc: &DescFile) -> Result<String, writer::DescWriterError> {
    let mut buffer = Vec::new();
    let mut writer = DescWriter::to_writer(&mut buffer);
    writer.write(desc)?;
    Ok(String::from_utf8(buffer).unwrap())
}
