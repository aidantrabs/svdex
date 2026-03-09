use anyhow::{Context, Result};
use ndarray::{Array1, Array2, s};
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

/// truncated svd: U[:,:k] * diag(S[:k]) * Vt[:k,:]
pub fn low_rank_approx(svd: &SvdResult, k: usize) -> Array2<f64> {
    let k = k.min(svd.s.len());

    let u_k = svd.u.slice(s![.., ..k]).to_owned();
    let s_k = &svd.s.slice(s![..k]);
    let vt_k = svd.vt.slice(s![..k, ..]).to_owned();

    let u_scaled = &u_k * s_k;
    u_scaled.dot(&vt_k)
}
