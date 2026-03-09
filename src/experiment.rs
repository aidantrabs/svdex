use anyhow::Result;
use ndarray::Array2;
use std::path::Path;

use crate::compression::{compress_with_svds, compute_channel_svds};
use crate::image_io::{channels_to_image, save_image};
use crate::metrics::{compute_report, MetricsReport};

pub fn run_experiment(
    channels: &[Array2<f64>; 3],
    ranks: &[usize],
    output_dir: &Path,
) -> Result<Vec<MetricsReport>> {
    println!("computing svd for all channels (once)...");
    let svds = compute_channel_svds(channels)?;

    let max_rank = channels[0].nrows().min(channels[0].ncols());
    let mut reports = Vec::new();

    println!("\n{:-<70}", "");
    println!(
        "  {:>4} | {:>8} | {:>10} | {:>8}",
        "rank", "ratio", "mse", "psnr"
    );
    println!("{:-<70}", "");

    for &k in ranks {
        if k > max_rank {
            println!("  skipping rank {k} (max is {max_rank})");
            continue;
        }

        let compressed = compress_with_svds(&svds, k);
        let report = compute_report(channels, &compressed, k);
        println!("{report}");

        let img = channels_to_image(&compressed);
        let out_path = output_dir.join(format!("compressed_k{k}.png"));
        save_image(&img, &out_path)?;

        reports.push(report);
    }
    println!("{:-<70}", "");

    Ok(reports)
}
