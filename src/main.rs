use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use svdex::cli::{Cli, Command};
use svdex::compression::compress_image;
use svdex::experiment::run_experiment;
use svdex::image_io::{channels_to_image, image_to_channels, load_image, save_image};
use svdex::matrix::channel_stats;
use svdex::metrics::compute_report;
use svdex::svd::compute_svd;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Info { image } => cmd_info(&image),
        Command::Compress { image, k, output } => cmd_compress(&image, k, output),
        Command::Experiment { image, ranks } => cmd_experiment(&image, &ranks),
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

    println!("\ncomputing svd for singular value preview...");
    for (i, ch) in channels.iter().enumerate() {
        let svd = compute_svd(ch)?;
        let n = svd.s.len().min(10);
        let top: Vec<String> = svd.s.iter().take(n).map(|v| format!("{v:.1}")).collect();
        println!("  {} top-{n} singular values: [{}]", names[i], top.join(", "));
    }

    Ok(())
}

fn cmd_compress(path: &PathBuf, k: usize, output: Option<PathBuf>) -> Result<()> {
    let img = load_image(path)?;
    let (w, h) = img.dimensions();
    println!("compressing {} ({w}x{h}) at rank {k}...", path.display());

    let channels = image_to_channels(&img);
    let compressed = compress_image(&channels, k)?;

    let report = compute_report(&channels, &compressed, k);
    println!("\n{report}");

    let out_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("output/compressed/compressed_k{k}.png"))
    });
    let result_img = channels_to_image(&compressed);
    save_image(&result_img, &out_path)?;
    println!("saved to {}", out_path.display());

    Ok(())
}

fn cmd_experiment(path: &PathBuf, ranks: &[usize]) -> Result<()> {
    let img = load_image(path)?;
    let (w, h) = img.dimensions();
    println!(
        "running experiment on {} ({w}x{h}) with ranks {:?}",
        path.display(),
        ranks
    );

    let channels = image_to_channels(&img);
    let out_dir = PathBuf::from("output/compressed");
    run_experiment(&channels, ranks, &out_dir)?;

    Ok(())
}
