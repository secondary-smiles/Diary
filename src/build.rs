use clap::Parser;
use pulldown_cmark::Options;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Parser)]
pub struct Build {
    date: Option<String>,

    /// Directory to write built HTML file to.
    #[arg(short, long, default_value = ".")]
    output: PathBuf,

    /// Convert all diary entries to HTML
    #[arg(short, long)]
    all: bool,

    /// Path to custom css to use.
    #[arg(short, long)]
    css: Option<Vec<PathBuf>>,

    /// Path to custom javascript to use.
    #[arg(short, long)]
    script: Option<Vec<PathBuf>>,

    /// The title of the generated document.
    #[arg(short, long)]
    title: Option<String>,
}

pub async fn build(args: Build) -> eyre::Result<()> {
    let config: crate::config::Config = confy::load("diary", None)?;

    let mut css_tags = vec![];
    let styles = args
        .css
        .clone()
        .unwrap_or(config.build.css.unwrap_or(vec![]));
    for style in styles {
        let mut s = String::new();
        File::open(style).await?.read_to_string(&mut s).await?;
        css_tags.push(el("style", None, s));
    }

    let mut script_tags = vec![];
    let scripts = args
        .script
        .clone()
        .unwrap_or(config.build.script.unwrap_or(vec![]));
    for script in scripts {
        let mut s = String::new();
        File::open(script).await?.read_to_string(&mut s).await?;
        script_tags.push(el("script", None, s));
    }
    let date = crate::util::pick_date(args.date.clone());
    if args.all {
        let entries = all_entries(config.location)?;
        for entry in entries {
            let mut contents = String::new();
            File::open(entry.clone())
                .await?
                .read_to_string(&mut contents)
                .await?;
            let parsed = parse(contents);
            let page = make_page(&css_tags, &script_tags, args.title.clone(), parsed).await?;
            if args.output.to_str().unwrap() == "-" {
                println!("{page}");
            } else {
                let mut path = args.output.clone();
                path.push(entry.file_name().unwrap());
                path.set_extension("html");
                File::create(path).await?.write_all(page.as_bytes()).await?;
            }
        }
    } else {
        let contents = parse(crate::util::get_entry_string(date).await?);
        let page = make_page(&css_tags, &script_tags, args.title, contents).await?;
        if args.output.to_str().unwrap() == "-" {
            println!("{page}");
        } else {
            let mut path = args.output.clone();
            let mut file_path = crate::util::get_entry_file_name(date);
            file_path.set_extension("html");
            path = path.join(file_path);
            File::create(path).await?.write_all(page.as_bytes()).await?;
        }
    }
    Ok(())
}

fn all_entries(base: PathBuf) -> eyre::Result<Vec<PathBuf>> {
    let mut files = vec![];

    let listing = fs::read_dir(base)?;
    for entry in listing {
        let entry = entry?;
        if entry.path().is_dir() {
            files.append(&mut all_entries(entry.path())?);
        } else {
            files.push(entry.path());
        }
    }
    Ok(files)
}

async fn make_page(
    css_tags: &Vec<String>,
    script_tags: &Vec<String>,
    title: Option<String>,
    contents: String,
) -> eyre::Result<String> {
    let config: crate::config::Config = confy::load("diary", None)?;
    let html = html(
        vec![
            el(
                "head",
                None,
                vec![
                    el("title", None, title.unwrap_or("Diary".to_string())),
                    css_tags.join("\n"),
                    script_tags.join("\n"),
                    config.build.frontmatter.render().await?,
                ]
                .join("\n"),
            ),
            el(
                "body",
                None,
                vec![contents, config.build.endmatter.render().await?].join("\n"),
            ),
        ]
        .join("\n"),
    );

    Ok(html)
}

fn parse(contents: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = pulldown_cmark::Parser::new_ext(contents.as_str(), options);

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    output
}

fn html(content: String) -> String {
    format!("<!DOCTYPE html>\n<html>\n{content}\n</html>")
}
fn el(el: &str, options: Option<&str>, content: String) -> String {
    format!("<{el} {}>{content}</{el}>", options.unwrap_or_default())
}
