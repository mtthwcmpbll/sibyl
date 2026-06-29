use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::court::CourtCardId;
use crate::suit::Suit;

/// A personal, sensitive string (the owner's deck-style text). Its `Debug` representation is
/// redacted so personal data never leaks into logs (Principle VI).
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
#[serde(rename_all = "camelCase")]
pub struct SuitIcon {
    pub suit: Suit,
    pub image_key: String,
}

/// One cohesive candidate covering every suit; the owner selects exactly one.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct DeckCreationSession {
    pub id: String,
    pub seed: u64,
    /// Authoritative suit-iconography rules — an app-defined resource, not owner input.
    pub iconography_rules: String,
    pub deck_style_text: Personal,
    pub style_options: Vec<StyleOption>,
    pub chosen_style_option_id: Option<String>,
    pub style_guide: Option<CardStyleGuide>,
    pub sample_card: Option<SampleCard>,
    pub status: SessionStatus,
    /// How many style-option rounds have been generated (FR-006). Drives reproducible
    /// re-rolls: each round derives a fresh sub-seed from the session seed.
    pub round: u32,
    /// Provenance accumulated across the session, finalized into the deck on approval (FR-014).
    pub provenance: GenerationProvenance,
}

impl DeckCreationSession {
    pub fn new(
        id: impl Into<String>,
        seed: u64,
        rules: impl Into<String>,
        style: Personal,
    ) -> Self {
        let rules = rules.into();
        let provenance = GenerationProvenance {
            seed,
            iconography_rules: rules.clone(),
            deck_style_text: style.clone(),
            ..Default::default()
        };
        Self {
            id: id.into(),
            seed,
            iconography_rules: rules,
            deck_style_text: style,
            style_options: Vec::new(),
            chosen_style_option_id: None,
            style_guide: None,
            sample_card: None,
            status: SessionStatus::Inputs,
            round: 0,
            provenance,
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
#[serde(rename_all = "camelCase")]
pub struct GenerationProvenance {
    pub seed: u64,
    /// Authoritative suit-iconography rules used (app resource, not personal).
    pub iconography_rules: String,
    pub deck_style_text: Personal,
    pub chosen_style_option_id: Option<String>,
    pub court_card_shown: Option<CourtCardId>,
    pub provider_id: String,
    pub model_id: String,
    /// The composed prompts actually sent (rules authoritative over style, FR-019).
    pub prompts: BTreeMap<String, String>,
}

/// Normalized rectangle (0–1 coordinates) within a card canvas.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }
}

/// Front-face layout: where the suit icon, card imagery, and title sit (normalized).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontLayout {
    pub aspect: f32,
    pub suit_icon: Rect,
    pub card_imagery: Rect,
    pub title: Rect,
}

/// A region designated for animated shader ("UV coating") effects — this feature only
/// *designates* it; the presentation layer applies the effect later.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderArea {
    pub id: String,
    pub rect: Rect,
    pub kind: String,
}

/// A decorative flourish placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlourishRef {
    pub asset_key: String,
    pub rect: Rect,
    pub rotation: f32,
}

/// The card-system definition derived from the chosen style (FR-007/008). Versioned so prior
/// cards stay attributable to the identity that produced them (Principle IV).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardStyleGuide {
    pub id: String,
    pub version: u32,
    pub derived_from_style_option_id: String,
    pub prompt_used: String,
    pub border_chrome_key: String,
    pub card_back_key: String,
    pub card_front_layout: FrontLayout,
    pub shader_areas: Vec<ShaderArea>,
    pub flourishes: Vec<FlourishRef>,
}

/// Approval outcome for the sample card (FR-011).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Approval {
    Pending,
    Approved,
    Rejected,
}

/// The single composed court/face card preview (FR-009/010; always a court card).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleCard {
    pub court_card: CourtCardId,
    pub card_imagery_key: String,
    pub front_key: String,
    pub back_key: String,
    pub approval: Approval,
}

/// The durable, approved deck identity — the prepared-state bundle future features consume.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    pub id: String,
    pub active: bool,
    pub suit_icons: Vec<SuitIcon>,
    pub style_guide: CardStyleGuide,
    pub provenance: GenerationProvenance,
}
