use thiserror::Error;

/// Orchestration-level errors. Carries a `retryable` hint so the wizard can offer retry
/// (FR-017) without losing the owner's inputs.
#[derive(Debug, Error)]
pub enum SibylError {
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
    #[error("no style guide generated")]
    NoStyleGuide,
    #[error("nothing to approve")]
    NothingToApprove,
    #[error("invalid state: {0}")]
    InvalidState(String),
}

impl SibylError {
    pub fn retryable(&self) -> bool {
        matches!(
            self,
            SibylError::Provider {
                retryable: true,
                ..
            }
        )
    }
}

impl From<providers::ProviderError> for SibylError {
    fn from(e: providers::ProviderError) -> Self {
        SibylError::Provider {
            message: e.message,
            retryable: e.retryable,
        }
    }
}

impl From<storage::StoreError> for SibylError {
    fn from(e: storage::StoreError) -> Self {
        SibylError::Storage(e.to_string())
    }
}

impl From<render::RenderError> for SibylError {
    fn from(e: render::RenderError) -> Self {
        SibylError::Render(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SibylError>;
