//! Provider-agnostic AI seam (constitution Principle I).
//!
//! All LLM/image generation flows through [`TextProvider`] / [`ImageProvider`]. The
//! [`FakeProvider`] is the deterministic default for tests and local dev. No vendor type
//! appears outside an adapter implementing these traits.

pub mod claude;
pub mod error;
pub mod fake;
pub mod traits;

pub use claude::ClaudeCliProvider;
pub use error::{ProviderError, ProviderErrorCode, ProviderId};
pub use fake::FakeProvider;
pub use traits::{
    GenParams, ImageBytes, ImageProvider, ImageRequest, Size, TextProvider, TextRequest,
    TextResponse,
};
