use domain::court::{CourtCardId, CourtRank};
use domain::suit::Suit;

#[test]
fn there_are_exactly_sixteen_court_cards() {
    let all = CourtCardId::all();
    assert_eq!(all.len(), 16);

    // All unique.
    let mut seen = std::collections::HashSet::new();
    for c in &all {
        assert!(seen.insert((c.rank, c.suit)), "duplicate court card {c}");
    }
}

#[test]
fn court_card_slug_is_rank_of_suit() {
    let c = CourtCardId::new(CourtRank::Queen, Suit::Cups);
    assert_eq!(c.slug(), "queen-of-cups");
}

#[test]
fn standard_suit_set_has_four() {
    assert_eq!(Suit::STANDARD.len(), 4);
}
