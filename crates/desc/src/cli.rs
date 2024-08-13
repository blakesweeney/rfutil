use std::{fs::File, path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use desc_lib::{edit::Edit, rna_type::RnaType};

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Apply the changes in a JSON file
    Apply {
        /// The JSON file to use
        filename: PathBuf,
    },

    /// Change the gathering threshold of the DESC file
    Rethreshold {
        /// The new cutoff to use
        cutoff: f64,
    },

    /// Update the description of the family
    Describe {
        /// The new description
        description: Vec<String>,
    },

    /// Update the comment of the family
    Comment {
        /// The new comment.
        comment: Option<String>,
    },

    /// Change the Rfam RNA type.
    Reclassify {
        /// The new RNA type
        rna_type: String,
    },

    /// Update the wiki article name
    UpdateWiki {
        /// The new article name or URL.
        article: String,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Do not actually update the file.
    ///
    /// If this is given the file will be written to stdout, not actually edited on disk.
    #[arg(short, long, default_value = "false")]
    dry_run: bool,

    /// The path to the DESC file to edit.
    ///
    /// This file will be edited in place.
    filename: PathBuf,

    /// The command to run.
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn perform(&self) -> Result<()> {
        let mut desc = desc_lib::from_path(&self.filename)?;
        let edit = match &self.command {
            Command::Comment { comment: _ } => todo!(),
            Command::Describe { description } => {
                let new_description = description.join(" ");
                assert!(!new_description.is_empty(), "Must give a description");
                Edit::Description(new_description)
            }
            Command::Rethreshold { cutoff } => Edit::GatheringThreshold(cutoff.clone()),
            Command::UpdateWiki { article: _ } => todo!(),
            Command::Reclassify { rna_type } => {
                let rtype = RnaType::from_str(rna_type)
                    .with_context(|| format!("Failed to find RNA type for `{}`", &rna_type))?;
                Edit::RnaType(rtype)
            }
            Command::Apply { filename } => {
                let rdr = File::open(&filename)?;
                serde_json::from_reader(rdr)?
            }
        };
        desc.edit(edit)?;
        if !self.dry_run {
            desc_lib::to_path(&self.filename, &desc)?;
        } else {
            let text = desc_lib::to_text(&desc)?;
            println!("{}", text);
        }
        Ok(())
    }
}
