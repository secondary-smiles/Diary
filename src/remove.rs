use clap::Parser;
use tokio::fs;

#[derive(Parser, Debug)]
pub struct Remove {
    date: Option<String>,
}

pub async fn remove(args: Remove) -> eyre::Result<()> {
    let date = crate::util::pick_date(args.date);
    let path = crate::util::get_entry_path(date)?;

    fs::remove_file(path).await?;
    Ok(())
}
