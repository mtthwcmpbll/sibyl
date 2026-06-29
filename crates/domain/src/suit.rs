use serde::{Deserialize, Serialize};

/// A tarot suit. The standard set is the default; the iconography rules may, in principle,
/// describe a different set, but the app ships with the standard four (spec Assumptions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Suit {
    Cups,
    Wands,
    Swords,
    Pentacles,
}

impl Suit {
    /// The standard four tarot suits, in canonical order.
    pub const STANDARD: [Suit; 4] = [Suit::Cups, Suit::Wands, Suit::Swords, Suit::Pentacles];

    pub fn as_str(&self) -> &'static str {
        match self {
            Suit::Cups => "cups",
            Suit::Wands => "wands",
            Suit::Swords => "swords",
            Suit::Pentacles => "pentacles",
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
