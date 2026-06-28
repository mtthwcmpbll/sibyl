use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::court::CourtCardId;
use crate::suit::Suit;

/// A personal, sensitive string (iconography rules, deck-style text). Its `Debug`
/// representation is redacted so personal data never leaks into logs (Principle VI).
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Personal(pub String);

impl Personal {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl std::fmt::Debug for Personal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never render the contents — show only that a value is present.
        if self.0.is_empty() {
            f.write_str("Personal(<empty>)")
        } else {
            f.write_str("Personal(<redacted>)")
        }
    }
}

impl From<&str> for Personal {
    fn from(s: &str) -> Self {
        Personal(s.to_string())
    }
}

impl From<String> for Personal {
    fn from(s: String) -> Self {
        Personal(s)
    }
}

/// One suit's generated icon, referenced by storage key (never embedded).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SuitIcon {
    pub suit: Suit,
    pub image_key: String,
}

/// One cohesive candidate covering every suit; the owner selects exactly one.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StyleOption {
    pub id: String,
    pub label: String,
    pub suit_icons: Vec<SuitIcon>,
    pub prompt_used: String,
}

impl StyleOption {
    /// A style option is valid only if it has an icon for every suit in the active set.
    pub fn covers_all_suits(&self, suits: &[Suit]) -> bool {
        suits
            .iter()
            .all(|s| self.suit_icons.iter().any(|i| i.suit == *s))
    }
}

/// Where the wizard is. Only `Approved` produces an active deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionStatus {
    Inputs,
    ChoosingStyle,
    StyleGuide,
    Review,
    Approved,
    Discarded,
}

/// The in-progress deck-creation wizard state (draft). Never "active".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckCreationSession {
    pub id: String,
    pub seed: u64,
    pub iconography_rules: Personal,
    pub deck_style_text: Personal,
    pub style_options: Vec<StyleOption>,
    pub chosen_style_option_id: Option<String>,
    pub status: SessionStatus,
    /// How many style-option rounds have been generated (FR-006). Drives reproducible
    /// re-rolls: each round derives a fresh sub-seed from the session seed.
    pub round: u32,
}

impl DeckCreationSession {
    pub fn new(id: impl Into<String>, seed: u64, rules: Personal, style: Personal) -> Self {
        Self {
            id: id.into(),
            seed,
            iconography_rules: rules,
            deck_style_text: style,
            style_options: Vec::new(),
            chosen_style_option_id: None,
            status: SessionStatus::Inputs,
            round: 0,
        }
    }

    pub fn chosen_option(&self) -> Option<&StyleOption> {
        let id = self.chosen_style_option_id.as_ref()?;
        self.style_options.iter().find(|o| &o.id == id)
    }
}

/// Recorded inputs + identifiers sufficient to explain and regenerate an artifact
/// (FR-014; Principles V/VI). Personal fields are redacted in Debug output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenerationProvenance {
    pub seed: u64,
    pub iconography_rules: Personal,
    pub deck_style_text: Personal,
    pub chosen_style_option_id: Option<String>,
    pub court_card_shown: Option<CourtCardId>,
    pub provider_id: String,
    pub model_id: String,
    /// The composed prompts actually sent (rules authoritative over style, FR-019).
    pub prompts: BTreeMap<String, String>,
}

/// The durable, approved deck identity — the prepared-state bundle future features consume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: String,
    pub active: bool,
    pub suit_icons: Vec<SuitIcon>,
    pub provenance: GenerationProvenance,
}
