use async_trait::async_trait;

use crate::error::{ProviderError, ProviderId};

/// Provider-neutral generation parameters. Adapters map these onto vendor params internally;
/// no vendor field name leaks out (Principle I).
#[derive(Debug, Clone, Default)]
pub struct GenParams {
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            width: 512,
            height: 512,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextRequest {
    /// System-level, authoritative instruction (e.g., the iconography rules).
    pub system: String,
    pub prompt: String,
    pub seed: u64,
    pub params: GenParams,
}

#[derive(Debug, Clone)]
pub struct TextResponse {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ImageRequest {
    pub prompt: String,
    pub seed: u64,
    pub size: Size,
    pub params: GenParams,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageBytes {
    pub mime: String,
    pub bytes: Vec<u8>,
}

/// Text generation (e.g., composing an image prompt from rules + style).
#[async_trait]
pub trait TextProvider: Send + Sync {
    async fn complete(&self, req: TextRequest) -> Result<TextResponse, ProviderError>;
    fn id(&self) -> ProviderId;
}

/// Image generation (suit icons, style-guide art, just-in-time card imagery).
#[async_trait]
pub trait ImageProvider: Send + Sync {
    async fn generate(&self, req: ImageRequest) -> Result<ImageBytes, ProviderError>;
    fn id(&self) -> ProviderId;
}
