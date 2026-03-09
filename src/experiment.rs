use anyhow::Result;
use ndarray::Array2;
use std::path::Path;

use crate::compression::{compress_with_svds, compute_channel_svds};
use crate::image_io::{channels_to_image, save_image};
use crate::metrics::{compute_report, MetricsReport};
use crate::svd::SvdResult;

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

    plot_sv_decay(&svds)?;

    Ok(reports)
}

fn plot_sv_decay(svds: &[SvdResult; 3]) -> Result<()> {
    use plotters::prelude::*;

    let out_path = "experiments/sv_decay.png";
    std::fs::create_dir_all("experiments")?;

    let colors = [RED, GREEN, BLUE];
    let labels = ["red", "green", "blue"];

    let max_len = svds.iter().map(|s| s.s.len()).max().unwrap_or(0);
    let max_val = svds
        .iter()
        .flat_map(|s| s.s.iter())
        .cloned()
        .fold(0.0f64, f64::max);

    let root = BitMapBackend::new(out_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("singular value decay", ("sans-serif", 24))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0usize..max_len, 0.0..max_val * 1.05)?;

    chart
        .configure_mesh()
        .x_desc("index")
        .y_desc("singular value")
        .draw()?;

    for (i, svd) in svds.iter().enumerate() {
        let data: Vec<(usize, f64)> = svd.s.iter().enumerate().map(|(j, &v)| (j, v)).collect();
        chart
            .draw_series(LineSeries::new(data, colors[i].stroke_width(2)))?
            .label(labels[i])
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], colors[i].stroke_width(2))
            });
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("\nsingular value decay plot saved to {out_path}");
    Ok(())
}
