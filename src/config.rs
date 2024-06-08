use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub location: PathBuf,
    pub pager: Option<String>,
    pub editor: Option<String>,

    pub entry: Entry,
    pub snippet: Snippet,
    pub build: Build,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub frontmatter: MatterContent,
    pub endmatter: MatterContent,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub frontmatter: MatterContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    pub css: Option<Vec<PathBuf>>,
    pub script: Option<Vec<PathBuf>>,
    pub frontmatter: MatterContent,
    pub endmatter: MatterContent,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MatterContent {
    pub content: Option<String>,
    pub cmd: Option<String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        let mut path = home::home_dir().unwrap();
        path.push("diary");

        Self {
            location: path,
            pager: std::env::var("PAGER").ok(),
            editor: std::env::var("EDITOR").ok(),
            entry: Entry {
                frontmatter: MatterContent {
                    content: None,
                    cmd: Some("printf \"# %s's Diary\n\" \"$(whoami)\"".to_string()),
                },
            },
            snippet: Snippet {
                frontmatter: MatterContent {
                    content: Some("\n---\n".to_string()),
                    cmd: Some("printf \"## At %s\" \"$(date +%R)\"".to_string()),
                },
                endmatter: MatterContent {
                    content: Some("\n---\n".to_string()),
                    cmd: None,
                },
            },
            build: Build {
                css: None,
                script: None,
                frontmatter: MatterContent::default(),
                endmatter: MatterContent::default(),
            },
        }
    }
}

impl MatterContent {
    pub async fn render(&self) -> eyre::Result<String> {
        let mut value = self.content.clone().unwrap_or_default();
        if let Some(cmd) = &self.cmd {
            let output = Command::new("sh")
                .args(&["-c", cmd.as_str()])
                .output()
                .await?;
            value += &String::from_utf8(output.stdout).unwrap();
        }
        Ok(value.to_string())
    }
}
