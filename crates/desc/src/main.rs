use anyhow::Result;
use clap::Parser;

pub mod cli;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    args.perform()?;
    Ok(())
}
