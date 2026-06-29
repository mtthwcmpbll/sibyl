//! Pure domain model and logic for the personal tarot deck — no IO.
//!
//! Holds the suits, court cards, seeded RNG, and the deck-creation data model. Personal
//! fields use [`model::Personal`], whose `Debug` is redacted (constitution Principle VI).

pub mod court;
pub mod model;
pub mod rng;
pub mod suit;

pub use court::{CourtCardId, CourtRank};
pub use model::{
    Approval, CardStyleGuide, Deck, DeckCreationSession, FlourishRef, FrontLayout,
    GenerationProvenance, Personal, Rect, SampleCard, SessionStatus, ShaderArea, StyleOption,
    SuitIcon,
};
pub use suit::Suit;

/// Fixed seed used by tests and headless runs so the whole pipeline is reproducible
/// (Principles V, VII).
pub const FIXED_SEED: u64 = 0x5EED_C0DE_1234_5678;
