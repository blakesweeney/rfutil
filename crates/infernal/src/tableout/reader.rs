use std::convert::AsRef;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::{fs::File, io, path::Path};

use csv::StringRecord;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use serde::de::DeserializeOwned;

use super::traits::Tabular;
use super::{v1, v2, v3};

lazy_static! {
    static ref NOT_WS: Regex = Regex::new(r"[^ ]").unwrap();
}

#[derive(Debug, Error)]
pub enum ReaderError {
    #[error("Failed to read file {0:?}, cause: {1}")]
    FailedToOpenFile(PathBuf, std::io::Error),
}

#[derive(Debug, Error)]
pub enum HitsError {
    /// If an IO error occurs while reading a line from a tableout file
    #[error(transparent)]
    IoError(#[from] io::Error),

    /// If an error occurs parsing the data as a CSV from the tableout file
    #[error(transparent)]
    CsvError(#[from] csv::Error),
}

pub struct TabularReader<R> {
    inner: R,
}

pub struct Hits<R, H> {
    buf: String,
    reader: BufReader<R>,
    number_of_columns: usize,
    hits: PhantomData<H>,
}

pub fn decode(raw: &str) -> &str {
    if raw == "-" {
        return "";
    }
    raw
}

impl<R> TabularReader<R>
where
    R: io::Read,
{
    pub fn from_reader(reader: R) -> Self {
        Self { inner: reader }
    }
}

impl TabularReader<File> {
    pub fn from_file(file: File) -> TabularReader<File> {
        Self::from_reader(file)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<TabularReader<File>, ReaderError> {
        let file = File::open(&path)
            .map_err(|e| ReaderError::FailedToOpenFile(PathBuf::from(path.as_ref()), e))?;
        Ok(TabularReader::from_reader(file))
    }
}

impl<R> TabularReader<R> {
    pub fn new(reader: R) -> Self {
        Self { inner: reader }
    }
}

impl<R: io::BufRead> TabularReader<R> {
    pub fn into_tabular<T>(self) -> Hits<R, T>
    where
        T: Tabular + DeserializeOwned,
    {
        Hits {
            buf: String::new(),
            reader: BufReader::new(self.inner),
            number_of_columns: T::columns(),
            hits: PhantomData::<T>,
        }
    }

    pub fn into_v1(self) -> Hits<R, v1::Hit> {
        self.into_tabular()
    }

    pub fn into_v2(self) -> Hits<R, v2::Hit> {
        self.into_tabular()
    }

    pub fn into_v3(self) -> Hits<R, v3::Hit> {
        self.into_tabular()
    }
}

impl<R, H> Iterator for Hits<R, H>
where
    R: io::Read,
    H: Tabular + DeserializeOwned,
{
    type Item = Result<H, HitsError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Err(e) => Some(Err(e.into())),
            Ok(0) => None,
            Ok(_n) => {
                if self.buf.starts_with('#') {
                    return self.next();
                }
                let parts: Vec<&str> = NOT_WS
                    .splitn(&self.buf, self.number_of_columns)
                    .map(decode)
                    .collect();
                let record = StringRecord::from(parts);
                Some(record.deserialize(None).map_err(HitsError::from))
            }
        }
    }
}
