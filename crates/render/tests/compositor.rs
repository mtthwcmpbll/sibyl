use image::Rgba;
use render::{compose, Layer};

/// A tiny solid-color PNG used as a layer.
fn solid_png(w: u32, h: u32, color: [u8; 4]) -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(w, h, Rgba(color));
    let mut buf = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();
    buf.into_inner()
}

#[test]
fn compositing_is_deterministic() {
    let layer = solid_png(4, 4, [255, 0, 0, 255]);
    let a = compose(
        16,
        16,
        Rgba([0, 0, 0, 255]),
        &[Layer {
            png: &layer,
            x: 2,
            y: 2,
        }],
    )
    .unwrap();
    let b = compose(
        16,
        16,
        Rgba([0, 0, 0, 255]),
        &[Layer {
            png: &layer,
            x: 2,
            y: 2,
        }],
    )
    .unwrap();
    assert_eq!(a, b, "same inputs must produce byte-identical output");
    assert!(!a.is_empty());
}

#[test]
fn opaque_layer_covers_background() {
    let layer = solid_png(16, 16, [10, 20, 30, 255]);
    let bytes = compose(
        16,
        16,
        Rgba([200, 200, 200, 255]),
        &[Layer {
            png: &layer,
            x: 0,
            y: 0,
        }],
    )
    .unwrap();
    let decoded = image::load_from_memory(&bytes).unwrap().to_rgba8();
    assert_eq!(decoded.get_pixel(8, 8).0, [10, 20, 30, 255]);
}
