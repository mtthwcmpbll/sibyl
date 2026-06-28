use std::io::Cursor;

use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("decode error: {0}")]
    Decode(String),
    #[error("encode error: {0}")]
    Encode(String),
}

pub type Result<T> = std::result::Result<T, RenderError>;

/// A layer to composite onto the canvas at an integer offset.
pub struct Layer<'a> {
    pub png: &'a [u8],
    pub x: i64,
    pub y: i64,
}

/// Canonical, deterministic CPU compositing (constitution Principles IV/V). Produces the
/// card image bytes that the presentation layer later animates. Pure-CPU, so output is
/// byte-stable across platforms.
pub fn compose(width: u32, height: u32, background: Rgba<u8>, layers: &[Layer]) -> Result<Vec<u8>> {
    let mut canvas = RgbaImage::from_pixel(width, height, background);

    for layer in layers {
        let top =
            image::load_from_memory(layer.png).map_err(|e| RenderError::Decode(e.to_string()))?;
        overlay(&mut canvas, &top, layer.x, layer.y);
    }

    let mut buf = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(canvas)
        .write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| RenderError::Encode(e.to_string()))?;
    Ok(buf.into_inner())
}

/// Simple source-over alpha blend of `top` onto `canvas` at `(ox, oy)`.
fn overlay(canvas: &mut RgbaImage, top: &DynamicImage, ox: i64, oy: i64) {
    let (tw, th) = top.dimensions();
    for ty in 0..th {
        for tx in 0..tw {
            let px = ox + tx as i64;
            let py = oy + ty as i64;
            if px < 0 || py < 0 || px >= canvas.width() as i64 || py >= canvas.height() as i64 {
                continue;
            }
            let src = top.get_pixel(tx, ty);
            let a = src[3] as u32;
            if a == 0 {
                continue;
            }
            let dst = canvas.get_pixel_mut(px as u32, py as u32);
            for c in 0..3 {
                let s = src[c] as u32;
                let d = dst[c] as u32;
                dst[c] = ((s * a + d * (255 - a)) / 255) as u8;
            }
            dst[3] = 255;
        }
    }
}
