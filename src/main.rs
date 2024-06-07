use clap::{Parser, Subcommand};

mod add;
mod config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(add::Add),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Add(args) => {
                add::add(args).await?;
            }
        }
    }

    Ok(())
}
