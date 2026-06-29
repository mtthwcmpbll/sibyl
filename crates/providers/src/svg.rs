//! Shared SVG handling for CLI providers that emit vector art (Claude, Antigravity): build a
//! "give me one SVG" prompt, extract the `<svg>` from a noisy response, and rasterize to PNG.

use crate::error::{ProviderError, ProviderErrorCode};
use crate::traits::ImageBytes;

/// Wrap a user image prompt with instructions to return a single self-contained SVG.
pub(crate) fn image_prompt(user_prompt: &str, w: u32, h: u32) -> String {
    format!(
        "You are generating vector art for a tarot deck. Output ONLY a single, self-contained, \
         valid SVG document: begin with `<svg` and end with `</svg>`, include a viewBox, and \
         size it {w}x{h}. No prose, no markdown code fences, no explanation. The SVG must be \
         fully self-contained: no external references, no <image>, and no <text> elements.\
         \n\nDepict: {}",
        user_prompt.trim()
    )
}

/// Parse a CLI response into PNG image bytes (extract the SVG, then rasterize).
pub(crate) fn to_png(raw: &str, w: u32, h: u32) -> Result<ImageBytes, ProviderError> {
    let svg = extract_svg(raw).ok_or_else(|| {
        ProviderError::new(
            ProviderErrorCode::BadResponse,
            "response did not contain an <svg> document".to_string(),
            true,
        )
    })?;
    Ok(ImageBytes {
        mime: "image/png".to_string(),
        bytes: rasterize_svg(svg, w, h)?,
    })
}

/// Extract the `<svg>…</svg>` document from a response that may include code fences or prose.
fn extract_svg(s: &str) -> Option<&str> {
    let start = s.find("<svg")?;
    let end = s.rfind("</svg>")? + "</svg>".len();
    (end > start).then(|| &s[start..end])
}

/// Rasterize an SVG string to PNG bytes at the target size (deterministic CPU render).
fn rasterize_svg(svg: &str, w: u32, h: u32) -> Result<Vec<u8>, ProviderError> {
    let bad = |m: String| ProviderError::new(ProviderErrorCode::BadResponse, m, true);

    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(svg, &opt).map_err(|e| bad(e.to_string()))?;

    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(w, h).ok_or_else(|| bad("invalid pixmap size".into()))?;

    let size = tree.size();
    let transform =
        resvg::tiny_skia::Transform::from_scale(w as f32 / size.width(), h as f32 / size.height());

    resvg::render(&tree, transform, &mut pixmap.as_mut());
    pixmap.encode_png().map_err(|e| bad(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_strips_fences_and_prose() {
        let raw = "Here you go:\n```svg\n<svg xmlns='http://www.w3.org/2000/svg' \
                   width='8' height='8'><rect width='8' height='8'/></svg>\n```\nDone.";
        let svg = extract_svg(raw).unwrap();
        assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"));
        assert!(!svg.contains("```"));
    }

    #[test]
    fn missing_svg_is_none() {
        assert!(extract_svg("no svg here, sorry").is_none());
    }

    #[test]
    fn to_png_rasterizes() {
        let raw = "<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16'>\
                   <circle cx='8' cy='8' r='7' fill='#1c2b4a'/></svg>";
        let img = to_png(raw, 32, 32).unwrap();
        assert_eq!(img.mime, "image/png");
        assert_eq!(&img.bytes[..4], &[0x89, b'P', b'N', b'G']);
    }
}
