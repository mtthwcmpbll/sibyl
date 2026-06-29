//! Deck-creation orchestration — the Preparation phase (constitution Principle III).
//!
//! Wires the provider seam, storage seam, and render together into the use-cases the wizard
//! drives. Holds no durable state itself; drafts and approved bundles live in the [`Store`].

pub mod approve;
pub mod error;
pub mod icons;
pub mod prompt;
pub mod sample;
pub mod style_guide;

use domain::DeckCreationSession;
use providers::{ImageProvider, TextProvider};
use storage::Store;

pub use approve::RestartInputs;
pub use error::{DeckForgeError, Result};
pub use icons::{GenerateIconStyles, IconStylesResult};
pub use prompt::compose_image_prompt;

/// Authoritative suit-iconography rules — defined by the application (not the owner) and
/// baked in as a resource. The owner supplies only their deck style; these rules enforce
/// consistent tarot-deck construction and take precedence over style (FR-001/FR-019).
pub const ICONOGRAPHY_RULES: &str = include_str!("../resources/iconography_rules.md");

/// Orchestrates deck creation against injected providers and storage. Construct it in the
/// host (e.g. the Tauri shell) from configuration; the core depends only on the traits.
pub struct DeckForge {
    text: Box<dyn TextProvider>,
    image: Box<dyn ImageProvider>,
    store: Box<dyn Store>,
}

impl DeckForge {
    pub fn new(
        text: Box<dyn TextProvider>,
        image: Box<dyn ImageProvider>,
        store: Box<dyn Store>,
    ) -> Self {
        Self { text, image, store }
    }

    pub(crate) fn session_key(id: &str) -> String {
        format!("decks/_drafts/{id}/session.json")
    }

    pub(crate) fn save_session(&self, session: &DeckCreationSession) -> Result<()> {
        let bytes = serde_json::to_vec_pretty(session)
            .map_err(|e| DeckForgeError::Storage(e.to_string()))?;
        self.store.put(&Self::session_key(&session.id), &bytes)?;
        Ok(())
    }

    pub fn load_session(&self, id: &str) -> Result<DeckCreationSession> {
        let key = Self::session_key(id);
        if !self.store.exists(&key) {
            return Err(DeckForgeError::UnknownSession(id.to_string()));
        }
        let bytes = self.store.get(&key)?;
        serde_json::from_slice(&bytes).map_err(|e| DeckForgeError::Storage(e.to_string()))
    }

    /// Resolve a storage key to raw bytes (used by the IPC layer's `get_asset`).
    pub fn get_asset(&self, key: &str) -> Result<Vec<u8>> {
        Ok(self.store.get(key)?)
    }
}
