use anyhow::Result;
use clap::Parser;

use svdex::cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Info { image } => {
            println!("info: {}", image.display());
        }
        Command::Compress { image, k, output } => {
            println!("compress: {} k={} output={:?}", image.display(), k, output);
        }
        Command::Experiment { image, ranks } => {
            println!("experiment: {} ranks={:?}", image.display(), ranks);
        }
    }

    Ok(())
}
