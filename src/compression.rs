use anyhow::Result;
use ndarray::Array2;

use crate::svd::{compute_svd, low_rank_approx, SvdResult};

const CHANNEL_NAMES: [&str; 3] = ["red", "green", "blue"];

pub fn compress_image(channels: &[Array2<f64>; 3], k: usize) -> Result<[Array2<f64>; 3]> {
    let mut compressed = Vec::with_capacity(3);
    for (i, channel) in channels.iter().enumerate() {
        println!("  computing svd for {} channel...", CHANNEL_NAMES[i]);
        let svd = compute_svd(channel)?;
        compressed.push(low_rank_approx(&svd, k));
    }
    Ok([
        compressed.remove(0),
        compressed.remove(0),
        compressed.remove(0),
    ])
}

pub fn compute_channel_svds(channels: &[Array2<f64>; 3]) -> Result<[SvdResult; 3]> {
    let mut svds = Vec::with_capacity(3);
    for (i, channel) in channels.iter().enumerate() {
        println!("  computing svd for {} channel...", CHANNEL_NAMES[i]);
        svds.push(compute_svd(channel)?);
    }
    Ok([svds.remove(0), svds.remove(0), svds.remove(0)])
}

pub fn compress_with_svds(svds: &[SvdResult; 3], k: usize) -> [Array2<f64>; 3] {
    [
        low_rank_approx(&svds[0], k),
        low_rank_approx(&svds[1], k),
        low_rank_approx(&svds[2], k),
    ]
}
