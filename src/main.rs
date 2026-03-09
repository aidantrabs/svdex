use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use svdex::cli::{Cli, Command};
use svdex::image_io::{image_to_channels, load_image};
use svdex::matrix::channel_stats;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Info { image } => cmd_info(&image),
        Command::Compress { image, k, output } => {
            println!("compress: {} k={} output={:?}", image.display(), k, output);
            Ok(())
        }
        Command::Experiment { image, ranks } => {
            println!("experiment: {} ranks={:?}", image.display(), ranks);
            Ok(())
        }
    }
}

fn cmd_info(path: &PathBuf) -> Result<()> {
    let img = load_image(path)?;
    let (w, h) = img.dimensions();
    println!("Image: {}", path.display());
    println!("Dimensions: {w} x {h}");

    let channels = image_to_channels(&img);
    let names = ["Red", "Green", "Blue"];

    for (i, ch) in channels.iter().enumerate() {
        let (min, max, mean) = channel_stats(ch);
        println!(
            "  {}: min={min:.0}, max={max:.0}, mean={mean:.1}",
            names[i]
        );
    }

    Ok(())
}
