use clap::Parser;
use rand::distributions::{Alphanumeric, DistString};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[derive(Debug, Parser)]
pub struct Add {
    file: Option<PathBuf>,

    #[arg(short, long)]
    editor: Option<String>,
    #[arg(short, long)]
    date: Option<String>,
}

pub async fn add(args: Add) -> eyre::Result<()> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let editor = args.editor.unwrap_or(config.editor.unwrap());
    let mut cmd = Command::new(&editor);
    let proc;
    let path;
    if let Some(file) = args.file {
        if !file.exists() {
            fs::File::create(file.clone()).await?;
        }
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
            "Editor '{}' exited with status code {}.",
            editor,
            proc.code().unwrap()
        )));
    }

    let mut diary_entry = String::new();
    fs::File::open(path)
        .await?
        .read_to_string(&mut diary_entry)
        .await?;
    add_diary_entry(diary_entry.trim().to_string(), args.date).await?;

    Ok(())
}

async fn add_diary_entry(contents: String, date: Option<String>) -> eyre::Result<()> {
    if contents.is_empty() {
        return Err(eyre::eyre!("Diary snippet empty, aborting."));
    }

    let config: crate::config::Config = confy::load("diary", None)?;
    let use_date = crate::util::pick_date(date);
    let path = crate::util::get_entry_path(use_date)?;
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
