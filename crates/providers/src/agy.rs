//! Provider adapter backed by the locally-installed `agy` CLI (Google Antigravity), using the
//! owner's logged-in session.
//!
//! Like the Claude adapter, Antigravity is an agentic text model — it does not emit raster
//! images — so this adapter asks for a self-contained **SVG** and rasterizes it to PNG (see
//! `crate::svg`). Text generation maps directly to `agy -p`.
//!
//! Adapter at the provider seam (Principle I): vendor-specific behavior stays here.

use std::process::{Command, Stdio};

use async_trait::async_trait;

use crate::error::{ProviderError, ProviderErrorCode, ProviderId};
use crate::traits::{
    ImageBytes, ImageProvider, ImageRequest, TextProvider, TextRequest, TextResponse,
};

/// Drives the `agy` binary in non-interactive print mode.
#[derive(Debug, Clone)]
pub struct AgyCliProvider {
    binary: String,
    /// Value passed to `--print-timeout` (Go duration, e.g. "5m").
    timeout: String,
}

impl AgyCliProvider {
    /// Construct from environment: `SIBYL_AGY_BIN` (default `agy`) and
    /// `SIBYL_AGY_TIMEOUT` (default `5m`).
    pub fn from_env() -> Self {
        Self {
            binary: std::env::var("SIBYL_AGY_BIN").unwrap_or_else(|_| "agy".to_string()),
            timeout: std::env::var("SIBYL_AGY_TIMEOUT").unwrap_or_else(|_| "5m".to_string()),
        }
    }

    /// Run `agy -p <prompt> --print-timeout <t>` and return stdout. Blocking; callers wrap it
    /// in `spawn_blocking`.
    fn run(&self, prompt: &str) -> Result<String, ProviderError> {
        let output = Command::new(&self.binary)
            .arg("-p")
            .arg(prompt)
            .arg("--print-timeout")
            .arg(&self.timeout)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| {
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
                format!("agy exited with {}: {}", output.status, stderr.trim()),
                true,
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

#[async_trait]
impl TextProvider for AgyCliProvider {
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
        ProviderId::new("antigravity", "agy")
    }
}

#[async_trait]
impl ImageProvider for AgyCliProvider {
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
        ProviderId::new("antigravity", "agy")
    }
}
