use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("key not found: {0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("serialization error: {0}")]
    Serde(String),
}

pub type Result<T> = std::result::Result<T, StoreError>;

/// Byte key/value persistence behind which every prepared-state artifact lives
/// (constitution Principle III). The default implementation is the local filesystem;
/// the backend is swappable (e.g., object store) without touching domain/render code.
pub trait Store: Send + Sync {
    fn get(&self, key: &str) -> Result<Vec<u8>>;
    fn put(&self, key: &str, bytes: &[u8]) -> Result<()>;
    fn exists(&self, key: &str) -> bool;
    /// List keys under a prefix (recursively).
    fn list(&self, prefix: &str) -> Result<Vec<String>>;
    fn delete_prefix(&self, prefix: &str) -> Result<()>;
}
