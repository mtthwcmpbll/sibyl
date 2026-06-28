use thiserror::Error;

/// Orchestration-level errors. Carries a `retryable` hint so the wizard can offer retry
/// (FR-017) without losing the owner's inputs.
#[derive(Debug, Error)]
pub enum DeckForgeError {
    #[error("provider failed: {message}")]
    Provider { message: String, retryable: bool },
    #[error("storage error: {0}")]
    Storage(String),
    #[error("render error: {0}")]
    Render(String),
    #[error("unknown session: {0}")]
    UnknownSession(String),
    #[error("unknown style option: {0}")]
    UnknownOption(String),
    #[error("no style selected")]
    NoStyleSelected,
}

impl DeckForgeError {
    pub fn retryable(&self) -> bool {
        matches!(
            self,
            DeckForgeError::Provider {
                retryable: true,
                ..
            }
        )
    }
}

impl From<providers::ProviderError> for DeckForgeError {
    fn from(e: providers::ProviderError) -> Self {
        DeckForgeError::Provider {
            message: e.message,
            retryable: e.retryable,
        }
    }
}

impl From<storage::StoreError> for DeckForgeError {
    fn from(e: storage::StoreError) -> Self {
        DeckForgeError::Storage(e.to_string())
    }
}

impl From<render::RenderError> for DeckForgeError {
    fn from(e: render::RenderError) -> Self {
        DeckForgeError::Render(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, DeckForgeError>;
