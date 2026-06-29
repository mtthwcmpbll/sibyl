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

// SVG extraction + rasterization is shared with the other vector-art CLI providers; see
// `crate::svg`.

/// Drives the `claude` binary in non-interactive print mode.
#[derive(Debug, Clone)]
pub struct ClaudeCliProvider {
    binary: String,
    model: Option<String>,
}

impl ClaudeCliProvider {
    /// Construct from environment: `SIBYL_CLAUDE_BIN` (default `claude`) and optional
    /// `SIBYL_CLAUDE_MODEL`.
    pub fn from_env() -> Self {
        Self {
            binary: std::env::var("SIBYL_CLAUDE_BIN").unwrap_or_else(|_| "claude".to_string()),
            model: std::env::var("SIBYL_CLAUDE_MODEL").ok(),
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
        let prompt = crate::svg::image_prompt(&req.prompt, w, h);

        let this = self.clone();
        let raw = tokio::task::spawn_blocking(move || this.run(&prompt))
            .await
            .map_err(|e| {
                ProviderError::new(ProviderErrorCode::Unavailable, e.to_string(), true)
            })??;

        crate::svg::to_png(&raw, w, h)
    }

    fn id(&self) -> ProviderId {
        ProviderId::new(
            "claude",
            self.model.clone().unwrap_or_else(|| "claude-code".into()),
        )
    }
}
