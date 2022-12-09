

mod day_01;
mod day_02;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1 { path: String },
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day1 { path } => {
            day_01::solve(path)?;
        }
    }
    Ok(())
}