use clap::Parser;
use tokio::process::Command;

#[derive(Parser, Debug)]
pub struct Edit {
    date: Option<String>,

    #[arg(short, long)]
    editor: Option<String>,
}

pub async fn edit(args: Edit) -> eyre::Result<()> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let date = crate::util::pick_date(args.date);
    let path = crate::util::get_entry_path(date)?;
    let editor = args.editor.unwrap_or(config.editor.unwrap());

    let proc = Command::new(&editor).arg(path).spawn()?.wait().await?;
    if !proc.success() {
        return Err(eyre::eyre!(format!(
            "Editor '{editor}' exited with status code {}.",
            proc.code().unwrap()
        )));
    }

    Ok(())
}
