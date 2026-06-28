use serde::{Deserialize, Serialize};

use crate::suit::Suit;

/// The four court (face) ranks. The Deck Creation sample card is always one of these
/// (clarification 2026-06-28): they offer the richest figure imagery for judging the deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourtRank {
    Page,
    Knight,
    Queen,
    King,
}

impl CourtRank {
    pub const ALL: [CourtRank; 4] = [
        CourtRank::Page,
        CourtRank::Knight,
        CourtRank::Queen,
        CourtRank::King,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            CourtRank::Page => "page",
            CourtRank::Knight => "knight",
            CourtRank::Queen => "queen",
            CourtRank::King => "king",
        }
    }
}

impl std::fmt::Display for CourtRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One of the 16 court cards, e.g. Queen of Cups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CourtCardId {
    pub rank: CourtRank,
    pub suit: Suit,
}

impl CourtCardId {
    pub fn new(rank: CourtRank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    /// All 16 court cards, in a stable order (rank-major).
    pub fn all() -> Vec<CourtCardId> {
        let mut out = Vec::with_capacity(16);
        for rank in CourtRank::ALL {
            for suit in Suit::STANDARD {
                out.push(CourtCardId { rank, suit });
            }
        }
        out
    }

    /// Stable slug, e.g. "queen-of-cups".
    pub fn slug(&self) -> String {
        format!("{}-of-{}", self.rank, self.suit)
    }
}

impl std::fmt::Display for CourtCardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.slug())
    }
}
