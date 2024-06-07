use clap::Parser;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long, env)]
    editor: String,
}

pub async fn add(args: Add) -> eyre::Result<()> {
    let mut cmd = Command::new(args.editor.clone());
    let proc;
    let path;
    if let Some(file) = args.file {
        proc = cmd.arg(file.clone()).spawn()?.wait().await?;
        path = file.to_str().unwrap().to_string();
    } else {
        let file = tempfile::NamedTempFile::new()?;
        path = format!("{}.md", file.into_temp_path().to_str().unwrap());
        proc = cmd.arg(path.clone()).spawn()?.wait().await?;
    }
    if !proc.success() {
        return Err(eyre::eyre!(format!(
            "'{}' exited with status code {}.",
            args.editor,
            proc.code().unwrap()
        )));
    }

    add_diary_entry(path).await?;

    Ok(())
}

async fn add_diary_entry(path: String) -> eyre::Result<()> {
    Ok(())
}
