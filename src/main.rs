use clap::{Parser, Subcommand};

mod add;
mod build;
mod config;
mod util;
mod view;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new snippet to today's entry.
    #[clap(visible_alias("a"))]
    Add(add::Add),

    /// View an entry from today or in the past.
    #[clap(visible_alias("v"))]
    View(view::View),

    /// Convert a diary entry to HTML.
    #[clap(visible_alias("b"))]
    Build(build::Build),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Commands::Add(args) => add::add(args).await?,
        Commands::View(args) => view::view(args).await?,
        Commands::Build(args) => build::build(args).await?,
    }

    Ok(())
}
