use anyhow::{Context, Result};
use image::{Rgb, RgbImage};
use ndarray::Array2;
use std::path::Path;

pub fn load_image(path: &Path) -> Result<RgbImage> {
    let img = image::open(path)
        .with_context(|| format!("Failed to open image: {}", path.display()))?;
    Ok(img.to_rgb8())
}

pub fn image_to_channels(img: &RgbImage) -> [Array2<f64>; 3] {
    let (w, h) = img.dimensions();
    let (rows, cols) = (h as usize, w as usize);

    let mut r = Array2::<f64>::zeros((rows, cols));
    let mut g = Array2::<f64>::zeros((rows, cols));
    let mut b = Array2::<f64>::zeros((rows, cols));

    for y in 0..rows {
        for x in 0..cols {
            let pixel = img.get_pixel(x as u32, y as u32);
            r[[y, x]] = pixel[0] as f64;
            g[[y, x]] = pixel[1] as f64;
            b[[y, x]] = pixel[2] as f64;
        }
    }

    [r, g, b]
}
