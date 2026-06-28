use async_trait::async_trait;
use std::io::Cursor;

use crate::error::{ProviderError, ProviderId};
use crate::traits::{
    ImageBytes, ImageProvider, ImageRequest, Size, TextProvider, TextRequest, TextResponse,
};

/// Deterministic, offline provider for tests and local dev. Output is a pure function of the
/// request (prompt/system + seed), so the whole pipeline reproduces identically with no
/// network and no credentials (Principles V, VII).
#[derive(Debug, Clone, Default)]
pub struct FakeProvider;

impl FakeProvider {
    pub fn new() -> Self {
        FakeProvider
    }
}

/// Stable FNV-1a hash (version/platform independent), used to derive deterministic output.
fn fnv1a(seed: u64, parts: &[&str]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325 ^ seed;
    for p in parts {
        for b in p.as_bytes() {
            hash ^= *b as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0x2c; // separator
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[async_trait]
impl TextProvider for FakeProvider {
    async fn complete(&self, req: TextRequest) -> Result<TextResponse, ProviderError> {
        // Echo the composition deterministically. The authority of `system` over `prompt`
        // is decided by the caller (compose_image_prompt); here we simply preserve it.
        let text = format!("{} || {}", req.system.trim(), req.prompt.trim());
        Ok(TextResponse {
            text: text.trim().to_string(),
        })
    }

    fn id(&self) -> ProviderId {
        ProviderId::new("fake", "fake-text-1")
    }
}

#[async_trait]
impl ImageProvider for FakeProvider {
    async fn generate(&self, req: ImageRequest) -> Result<ImageBytes, ProviderError> {
        let h = fnv1a(req.seed, &[&req.prompt]);
        let r = (h & 0xff) as u8;
        let g = ((h >> 8) & 0xff) as u8;
        let b = ((h >> 16) & 0xff) as u8;

        // Keep fakes tiny for fast tests, regardless of requested size.
        let Size { width, height } = req.size;
        let w = width.clamp(8, 64);
        let h_px = height.clamp(8, 64);

        let img = image::RgbaImage::from_pixel(w, h_px, image::Rgba([r, g, b, 255]));
        let mut buf = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut buf, image::ImageFormat::Png)
            .map_err(|e| {
                ProviderError::new(
                    crate::error::ProviderErrorCode::BadResponse,
                    e.to_string(),
                    false,
                )
            })?;

        Ok(ImageBytes {
            mime: "image/png".to_string(),
            bytes: buf.into_inner(),
        })
    }

    fn id(&self) -> ProviderId {
        ProviderId::new("fake", "fake-image-1")
    }
}
