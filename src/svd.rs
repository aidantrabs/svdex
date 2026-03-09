use anyhow::{Context, Result};
use ndarray::{Array1, Array2};
use ndarray_linalg::SVD;

pub struct SvdResult {
    pub u: Array2<f64>,
    pub s: Array1<f64>,
    pub vt: Array2<f64>,
}

pub fn compute_svd(matrix: &Array2<f64>) -> Result<SvdResult> {
    let (u, s, vt) = matrix
        .svd(true, true)
        .context("svd computation failed")?;

    Ok(SvdResult {
        u: u.context("svd did not produce u matrix")?,
        s,
        vt: vt.context("svd did not produce vt matrix")?,
    })
}
