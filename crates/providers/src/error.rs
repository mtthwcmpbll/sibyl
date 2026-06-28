use serde::{Deserialize, Serialize};

/// Provider-neutral error categories. Adapters map vendor failures onto these so callers
/// (and the wizard) can decide whether to retry — without seeing vendor types (Principle I).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderErrorCode {
    Unavailable,
    RateLimited,
    Timeout,
    BadResponse,
    Auth,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderError {
    pub code: ProviderErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl ProviderError {
    pub fn new(code: ProviderErrorCode, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(ProviderErrorCode::Unavailable, message, true)
    }
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "provider error [{:?}]: {}", self.code, self.message)
    }
}

impl std::error::Error for ProviderError {}

/// Stable, non-secret identifier of a provider/model, recorded in provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderId {
    pub provider: String,
    pub model: String,
}

impl ProviderId {
    pub fn new(provider: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            model: model.into(),
        }
    }
}
