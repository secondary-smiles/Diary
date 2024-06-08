use clap::Parser;
use tokio::process::Command;

#[derive(Debug, Parser)]
pub struct View {
    date: Option<String>,

    #[arg(short, long)]
    pager: Option<String>,

    /// Print entry directly to stdout.
    #[arg(short, long)]
    stdout: bool,
}

pub async fn view(args: View) -> eyre::Result<()> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let pager = args.pager.unwrap_or(config.pager.unwrap());
    let use_date = crate::util::pick_date(args.date);
    let path = crate::util::get_entry_path(use_date)?;

    if !path.exists() {
        return Err(eyre::eyre!(format!(
            "Diary entry for '{}' does not exist. (Expected path: {:#?})",
            use_date.date_naive(),
            path
        )));
    }

    if args.stdout {
        let contents = crate::util::get_entry_string(use_date).await?;
        println!("{}", contents);
        return Ok(());
    }

    let proc = Command::new(&pager).arg(path).spawn()?.wait().await?;
    if !proc.success() {
        return Err(eyre::eyre!(format!(
            "Pager '{}' exited with code {}",
            pager,
            proc.code().unwrap()
        )));
    }

    Ok(())
}
