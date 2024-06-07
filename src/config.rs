use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub location: PathBuf,

    pub entry: Matter,
    pub snippet: Matter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Matter {
    pub frontmatter: MatterContent,
    pub endmatter: MatterContent,
}

#[derive(Serialize, Deserialize, Debug)]
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
            entry: Matter {
                frontmatter: MatterContent {
                    content: None,
                    cmd: Some("printf \"# %s's Diary\n\n\" \"$(whoami)\"".to_string()),
                },
                endmatter: MatterContent {
                    content: None,
                    cmd: Some("printf \"> On %s\" \"$(date +%F)\n\"".to_string()),
                },
            },
            snippet: Matter {
                frontmatter: MatterContent {
                    content: Some("---\n".to_string()),
                    cmd: Some("printf \"## At %s\" \"$(date +%R)\n\"".to_string()),
                },
                endmatter: MatterContent {
                    content: Some("\n---".to_string()),
                    cmd: None,
                },
            },
        }
    }
}

impl Matter {}

impl MatterContent {
    pub async fn render(&self) -> eyre::Result<String> {
        let mut value = String::new();
        if let Some(cmd) = &self.cmd {
            let output = Command::new("sh")
                .args(&["-c", cmd.as_str()])
                .output()
                .await?;
        }
        Ok(String::new())
    }
}
