use domain::rng::{select_court_card, sub_seed};
use domain::FIXED_SEED;

#[test]
fn court_card_selection_is_deterministic_for_a_seed() {
    let a = select_court_card(FIXED_SEED);
    let b = select_court_card(FIXED_SEED);
    assert_eq!(a, b, "same seed must select the same court card");
}

#[test]
fn different_seeds_can_select_different_cards() {
    // Across a spread of seeds we must see more than one distinct court card (varied =
    // Surprise). This is overwhelmingly likely with 16 options.
    let mut seen = std::collections::HashSet::new();
    for s in 0..64u64 {
        seen.insert(select_court_card(s));
    }
    assert!(seen.len() > 1, "selection should vary across seeds");
}

#[test]
fn sub_seed_is_stable_and_label_sensitive() {
    assert_eq!(sub_seed(FIXED_SEED, "cups"), sub_seed(FIXED_SEED, "cups"));
    assert_ne!(sub_seed(FIXED_SEED, "cups"), sub_seed(FIXED_SEED, "wands"));
}
