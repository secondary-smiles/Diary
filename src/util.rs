use chrono::{DateTime, Datelike, Utc};
use chrono_english::{parse_date_string, Dialect};
use std::path::PathBuf;

pub fn pick_date(date: Option<String>) -> DateTime<Utc> {
    let mut use_date = Utc::now();
    if let Some(s) = date {
        let date_time = parse_date_string(s.as_str(), Utc::now(), Dialect::Us);
        if date_time.is_ok() {
            use_date = date_time.unwrap();
        }
    }

    use_date
}

pub fn get_entry_path(date: DateTime<Utc>) -> eyre::Result<PathBuf> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let mut path = config.location;
    path.push(date.year().to_string());
    path.push(format!("{}-{}-{}", date.year(), date.month(), date.day()));
    path.set_extension("md");

    Ok(path)
}
