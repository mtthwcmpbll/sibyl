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
pub use error::{Result, SibylError};
pub use icons::{GenerateIconStyles, IconStylesResult};
pub use prompt::build_image_prompt;

/// Authoritative, app-defined prompt rules — one per generated artifact type. Each is a
/// developer-editable resource that is **prepended** to the LLM prompt for that artifact and
/// takes precedence over the owner's deck-style direction (FR-001/FR-019). The owner supplies
/// only their deck style.
pub mod rules {
    /// Suit icons (Cups, Wands, Swords, Pentacles).
    pub const SUIT_ICONS: &str = include_str!("../resources/prompts/suit_icons.md");
    /// Card border / chrome frame.
    pub const CARD_BORDER: &str = include_str!("../resources/prompts/card_border.md");
    /// Card back (the uniform reverse shown face down).
    pub const CARD_BACK: &str = include_str!("../resources/prompts/card_back.md");
    /// Card imagery (the artwork/background on a card face).
    pub const BACKGROUND_IMAGE: &str = include_str!("../resources/prompts/background_image.md");
    /// Decorative flourishes.
    pub const FLOURISH: &str = include_str!("../resources/prompts/flourish.md");
}

/// Orchestrates deck creation against injected providers and storage. Construct it in the
/// host (e.g. the Tauri shell) from configuration; the core depends only on the traits.
pub struct Sibyl {
    /// Text provider — reserved for future text generation (e.g. weaving personal references
    /// at draw time). The current image-only Preparation pipeline does not use it.
    #[allow(dead_code)]
    text: Box<dyn TextProvider>,
    image: Box<dyn ImageProvider>,
    store: Box<dyn Store>,
}

impl Sibyl {
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
        let bytes =
            serde_json::to_vec_pretty(session).map_err(|e| SibylError::Storage(e.to_string()))?;
        self.store.put(&Self::session_key(&session.id), &bytes)?;
        Ok(())
    }

    pub fn load_session(&self, id: &str) -> Result<DeckCreationSession> {
        let key = Self::session_key(id);
        if !self.store.exists(&key) {
            return Err(SibylError::UnknownSession(id.to_string()));
        }
        let bytes = self.store.get(&key)?;
        serde_json::from_slice(&bytes).map_err(|e| SibylError::Storage(e.to_string()))
    }

    /// Resolve a storage key to raw bytes (used by the IPC layer's `get_asset`).
    pub fn get_asset(&self, key: &str) -> Result<Vec<u8>> {
        Ok(self.store.get(key)?)
    }
}
