use ndarray::Array2;
use std::fmt;

pub struct MetricsReport {
    pub rank: usize,
    pub compression_ratio: f64,
    pub mse: f64,
    pub psnr: f64,
}

impl fmt::Display for MetricsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  rank: {:>4} | ratio: {:>6.2}x | mse: {:>10.2} | psnr: {:>6.2} dB",
            self.rank, self.compression_ratio, self.mse, self.psnr
        )
    }
}

pub fn compression_ratio(h: usize, w: usize, k: usize, num_channels: usize) -> f64 {
    let original = (h * w * num_channels) as f64;
    // each channel stores u[:,:k] (h*k) + s[:k] (k) + vt[:k,:] (k*w)
    let compressed = (num_channels * (h * k + k + k * w)) as f64;
    original / compressed
}

pub fn mse(original: &[Array2<f64>; 3], compressed: &[Array2<f64>; 3]) -> f64 {
    let mut total = 0.0;
    let mut count = 0usize;
    for c in 0..3 {
        let diff = &original[c] - &compressed[c];
        total += diff.mapv(|v| v * v).sum();
        count += original[c].len();
    }
    total / count as f64
}

pub fn psnr(mse_val: f64, max_val: f64) -> f64 {
    if mse_val == 0.0 {
        return f64::INFINITY;
    }
    10.0 * (max_val * max_val / mse_val).log10()
}

pub fn compute_report(
    original: &[Array2<f64>; 3],
    compressed: &[Array2<f64>; 3],
    k: usize,
) -> MetricsReport {
    let h = original[0].nrows();
    let w = original[0].ncols();
    let ratio = compression_ratio(h, w, k, 3);
    let mse_val = mse(original, compressed);
    let psnr_val = psnr(mse_val, 255.0);
    MetricsReport {
        rank: k,
        compression_ratio: ratio,
        mse: mse_val,
        psnr: psnr_val,
    }
}
