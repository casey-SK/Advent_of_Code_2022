

mod day_01;
mod day_02;
mod day_03;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use color_eyre::eyre::Context;


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
    Day2 { path: String },
    Day3 { path: String },
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    color_eyre::install()?;

    match &cli.command {
        Commands::Day1 { path } => {
            day_01::solve(get_input(path).unwrap())?;
        }
        Commands::Day2 { path } => {
            day_02::solve(get_input(path).unwrap())?;
        }
        Commands::Day3 { path } => {
            day_03::solve(get_input(path).unwrap())?;
        }
    }
    Ok(())
}

fn get_input(arg_path: &String) -> color_eyre::Result<BufReader<File>> {
    let path = Path::new(&arg_path);
    let display = path.display();

    let file = File::open(path).wrap_err(format!("reading {:?}", display))?;

    let reader = BufReader::new(file);

    Ok(reader)
}

/// Used in an iterator to prevent silent errors
pub fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}