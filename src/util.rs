use chrono::{DateTime, Datelike, Local};
use chrono_english::{parse_date_string, Dialect};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub fn pick_date(date: Option<String>) -> DateTime<Local> {
    let mut use_date = Local::now();
    if let Some(s) = date {
        let date_time = parse_date_string(s.as_str(), Local::now(), Dialect::Us);
        if date_time.is_ok() {
            use_date = date_time.unwrap();
        }
    }

    use_date
}

pub fn get_entry_path(date: DateTime<Local>) -> eyre::Result<PathBuf> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let mut path = config.location;
    path.push(date.year().to_string());
    path.push(format!("{}-{}-{}", date.year(), date.month(), date.day()));
    path.set_extension("md");

    Ok(path)
}

pub fn get_entry_file_name(date: DateTime<Local>) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(format!("{}-{}-{}", date.year(), date.month(), date.day()));
    path.set_extension("md");

    path
}

pub async fn get_entry_string(date: DateTime<Local>) -> eyre::Result<String> {
    let path = crate::util::get_entry_path(date)?;

    if !path.exists() {
        return Err(eyre::eyre!(format!(
            "Diary entry for '{}' does not exist. (Expected path: {:#?})",
            date.date_naive(),
            path
        )));
    }

    let mut contents: String = String::new();
    File::open(path)
        .await?
        .read_to_string(&mut contents)
        .await?;

    Ok(contents)
}
