use chrono::{Datelike, Utc};
use clap::Parser;
use rand::distributions::{Alphanumeric, DistString};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long, env)]
    editor: String,
}

pub async fn add(args: Add) -> eyre::Result<()> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let mut cmd = Command::new(args.editor.clone());
    let proc;
    let path;
    if let Some(file) = args.file {
        proc = cmd.arg(file.clone()).spawn()?.wait().await?;
        path = file.to_str().unwrap().to_string();
    } else {
        let filename = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let mut file_path = PathBuf::from("/tmp");
        file_path.push(filename);
        file_path.set_extension("md");
        path = file_path.to_str().unwrap().to_string();
        proc = cmd.arg(path.clone()).spawn()?.wait().await?;
    }
    if !proc.success() {
        return Err(eyre::eyre!(format!(
            "'{}' exited with status code {}.",
            args.editor,
            proc.code().unwrap()
        )));
    }

    let mut diary_entry = String::new();
    fs::File::open(path)
        .await?
        .read_to_string(&mut diary_entry)
        .await?;
    add_diary_entry(diary_entry.trim().to_string()).await?;

    Ok(())
}

async fn add_diary_entry(contents: String) -> eyre::Result<()> {
    if contents.is_empty() {
        return Err(eyre::eyre!("Diary entry empty, aborting."));
    }

    let config: crate::config::Config = confy::load("diary", None)?;
    let now = Utc::now();
    let mut path = config.location;
    path.push(now.year().to_string());
    path.push(format!("{}-{}-{}", now.year(), now.month(), now.day()));
    path.set_extension("md");
    if !path.exists() {
        let frontmatter = config.entry.frontmatter.render().await?;
        fs::create_dir_all(path.parent().unwrap()).await?;
        let mut file = fs::File::create(path.clone()).await?;
        file.write_all(frontmatter.as_bytes()).await?;
    }
    let frontmatter = config.snippet.frontmatter.render().await?;
    let mut file = fs::OpenOptions::new().append(true).open(path).await?;
    file.write_all(frontmatter.as_bytes()).await?;
    file.write_all(contents.as_bytes()).await?;
    Ok(())
}
