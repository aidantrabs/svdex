use ndarray::Array2;

pub fn channel_stats(matrix: &Array2<f64>) -> (f64, f64, f64) {
    let min = matrix.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = matrix.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = matrix.mean().unwrap_or(0.0);
    (min, max, mean)
}
