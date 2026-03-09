use anyhow::{Context, Result};
use image::{ImageBuffer, Rgb, RgbImage};
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

pub fn channels_to_image(channels: &[Array2<f64>; 3]) -> RgbImage {
    let rows = channels[0].nrows();
    let cols = channels[0].ncols();

    ImageBuffer::from_fn(cols as u32, rows as u32, |x, y| {
        let r = channels[0][[y as usize, x as usize]].clamp(0.0, 255.0) as u8;
        let g = channels[1][[y as usize, x as usize]].clamp(0.0, 255.0) as u8;
        let b = channels[2][[y as usize, x as usize]].clamp(0.0, 255.0) as u8;
        Rgb([r, g, b])
    })
}

pub fn save_image(img: &RgbImage, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    img.save(path)
        .with_context(|| format!("Failed to save image: {}", path.display()))?;
    Ok(())
}
