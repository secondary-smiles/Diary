use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long, env)]
    editor: String,
}

pub async fn add(args: Add) -> eyre::Result<()> {
    Ok(())
}
