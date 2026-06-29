//! Provider adapter backed by the locally-installed `claude` CLI (Claude Code), using the
//! owner's logged-in consumer subscription.
//!
//! Note: Claude does not emit raster images, so this adapter asks Claude for a self-contained
//! **SVG** and rasterizes it to PNG for the compositing pipeline. That suits clean, emblematic
//! tarot iconography well. Text generation maps directly to `claude -p`.
//!
//! This is an adapter at the provider seam (Principle I): vendor-specific behavior stays here.

use std::process::{Command, Stdio};

use async_trait::async_trait;

use crate::error::{ProviderError, ProviderErrorCode, ProviderId};
use crate::traits::{
    ImageBytes, ImageProvider, ImageRequest, TextProvider, TextRequest, TextResponse,
};

/// Drives the `claude` binary in non-interactive print mode.
#[derive(Debug, Clone)]
pub struct ClaudeCliProvider {
    binary: String,
    model: Option<String>,
}

impl ClaudeCliProvider {
    /// Construct from environment: `DECKFORGE_CLAUDE_BIN` (default `claude`) and optional
    /// `DECKFORGE_CLAUDE_MODEL`.
    pub fn from_env() -> Self {
        Self {
            binary: std::env::var("DECKFORGE_CLAUDE_BIN").unwrap_or_else(|_| "claude".to_string()),
            model: std::env::var("DECKFORGE_CLAUDE_MODEL").ok(),
        }
    }

    /// Run `claude -p <prompt> --output-format text` and return stdout. Blocking; callers wrap
    /// it in `spawn_blocking`.
    fn run(&self, prompt: &str) -> Result<String, ProviderError> {
        let mut cmd = Command::new(&self.binary);
        cmd.arg("-p")
            .arg(prompt)
            .arg("--output-format")
            .arg("text")
            .stdin(Stdio::null());
        if let Some(model) = &self.model {
            cmd.arg("--model").arg(model);
        }

        let output = cmd.output().map_err(|e| {
            ProviderError::new(
                ProviderErrorCode::Unavailable,
                format!("failed to run `{}`: {e}", self.binary),
                true,
            )
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ProviderError::new(
                ProviderErrorCode::Unavailable,
                format!("claude exited with {}: {}", output.status, stderr.trim()),
                true,
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

#[async_trait]
impl TextProvider for ClaudeCliProvider {
    async fn complete(&self, req: TextRequest) -> Result<TextResponse, ProviderError> {
        let this = self.clone();
        let prompt = format!("{}\n\n{}", req.system.trim(), req.prompt.trim());
        let text = tokio::task::spawn_blocking(move || this.run(&prompt))
            .await
            .map_err(|e| {
                ProviderError::new(ProviderErrorCode::Unavailable, e.to_string(), true)
            })??;
        Ok(TextResponse {
            text: text.trim().to_string(),
        })
    }

    fn id(&self) -> ProviderId {
        ProviderId::new(
            "claude",
            self.model.clone().unwrap_or_else(|| "claude-code".into()),
        )
    }
}

#[async_trait]
impl ImageProvider for ClaudeCliProvider {
    async fn generate(&self, req: ImageRequest) -> Result<ImageBytes, ProviderError> {
        let (w, h) = (req.size.width.max(1), req.size.height.max(1));
        let prompt = format!(
            "You are generating vector art for a tarot deck. Output ONLY a single, \
             self-contained, valid SVG document: begin with `<svg` and end with `</svg>`, \
             include a viewBox, and size it {w}x{h}. No prose, no markdown code fences, no \
             explanation. The SVG must be fully self-contained: no external references, no \
             <image>, and no <text> elements.\n\nDepict: {}",
            req.prompt.trim()
        );

        let this = self.clone();
        let raw = tokio::task::spawn_blocking(move || this.run(&prompt))
            .await
            .map_err(|e| {
                ProviderError::new(ProviderErrorCode::Unavailable, e.to_string(), true)
            })??;

        let svg = extract_svg(&raw).ok_or_else(|| {
            ProviderError::new(
                ProviderErrorCode::BadResponse,
                "claude response did not contain an <svg> document".to_string(),
                true,
            )
        })?;

        let bytes = rasterize_svg(svg, w, h)?;
        Ok(ImageBytes {
            mime: "image/png".to_string(),
            bytes,
        })
    }

    fn id(&self) -> ProviderId {
        ProviderId::new(
            "claude",
            self.model.clone().unwrap_or_else(|| "claude-code".into()),
        )
    }
}

/// Extract the `<svg>…</svg>` document from a response that may include code fences or prose.
fn extract_svg(s: &str) -> Option<&str> {
    let start = s.find("<svg")?;
    let end = s.rfind("</svg>")? + "</svg>".len();
    if end > start {
        Some(&s[start..end])
    } else {
        None
    }
}

/// Rasterize an SVG string to PNG bytes at the target size (deterministic CPU render).
fn rasterize_svg(svg: &str, w: u32, h: u32) -> Result<Vec<u8>, ProviderError> {
    let bad = |m: String| ProviderError::new(ProviderErrorCode::BadResponse, m, true);

    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(svg, &opt).map_err(|e| bad(e.to_string()))?;

    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(w, h).ok_or_else(|| bad("invalid pixmap size".into()))?;

    let size = tree.size();
    let sx = w as f32 / size.width();
    let sy = h as f32 / size.height();
    let transform = resvg::tiny_skia::Transform::from_scale(sx, sy);

    resvg::render(&tree, transform, &mut pixmap.as_mut());
    pixmap.encode_png().map_err(|e| bad(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_svg_strips_fences_and_prose() {
        let raw = "Here you go:\n```svg\n<svg xmlns='http://www.w3.org/2000/svg' \
                   width='8' height='8'><rect width='8' height='8'/></svg>\n```\nDone.";
        let svg = extract_svg(raw).unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
        assert!(!svg.contains("```"));
    }

    #[test]
    fn rasterize_svg_produces_a_png() {
        let svg = "<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16'>\
                   <circle cx='8' cy='8' r='7' fill='#1c2b4a'/></svg>";
        let png = rasterize_svg(svg, 32, 32).unwrap();
        assert!(!png.is_empty());
        // PNG magic number.
        assert_eq!(&png[..4], &[0x89, b'P', b'N', b'G']);
    }

    #[test]
    fn missing_svg_is_none() {
        assert!(extract_svg("no svg here, sorry").is_none());
    }
}
