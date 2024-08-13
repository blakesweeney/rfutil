use clap::Subcommand;
use thiserror::Error;

pub enum DescFormat {
    Desc,
    Json,
    Text,
}

#[derive(Subcommand, Debug)]
pub struct GetCommand {
    pub format: GetFormat,
    items: Vec<Fields>,
}
