use domain::{CardStyleGuide, Deck, FrontLayout, GenerationProvenance, Rect, Suit, SuitIcon};
use storage::{read_active, read_active_deck, write_deck_and_activate, FsStore};

fn style_guide(id: &str) -> CardStyleGuide {
    CardStyleGuide {
        id: format!("sg-{id}"),
        version: 1,
        derived_from_style_option_id: "r0-opt0".into(),
        prompt_used: "p".into(),
        border_chrome_key: format!("decks/{id}/style-guide/border-chrome.png"),
        card_back_key: format!("decks/{id}/style-guide/card-back.png"),
        card_front_layout: FrontLayout {
            aspect: 0.66,
            suit_icon: Rect::new(0.0, 0.0, 0.2, 0.2),
            card_imagery: Rect::new(0.1, 0.15, 0.8, 0.6),
            title: Rect::new(0.0, 0.85, 1.0, 0.1),
        },
        shader_areas: vec![],
        flourishes: vec![],
    }
}

fn sample_deck(id: &str) -> Deck {
    Deck {
        id: id.to_string(),
        active: true,
        suit_icons: vec![SuitIcon {
            suit: Suit::Cups,
            image_key: format!("decks/{id}/icons/cups.png"),
        }],
        style_guide: style_guide(id),
        provenance: GenerationProvenance {
            seed: 7,
            provider_id: "fake".into(),
            model_id: "fake-image-1".into(),
            ..Default::default()
        },
    }
}

#[test]
fn no_active_deck_by_default() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());
    assert!(read_active(&store).unwrap().active_deck_id.is_none());
    assert!(read_active_deck(&store).unwrap().is_none());
}

#[test]
fn approving_writes_and_activates_single_deck() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());

    write_deck_and_activate(&store, &sample_deck("deck-1")).unwrap();
    let active = read_active_deck(&store).unwrap().unwrap();
    assert_eq!(active.id, "deck-1");

    // A second approval reassigns the single active pointer.
    write_deck_and_activate(&store, &sample_deck("deck-2")).unwrap();
    assert_eq!(
        read_active(&store).unwrap().active_deck_id.as_deref(),
        Some("deck-2")
    );
}
