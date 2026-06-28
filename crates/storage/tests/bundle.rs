use domain::{Deck, GenerationProvenance, Suit, SuitIcon};
use storage::{read_active, read_active_deck, write_deck_and_activate, FsStore};

fn sample_deck(id: &str) -> Deck {
    Deck {
        id: id.to_string(),
        active: true,
        suit_icons: vec![SuitIcon {
            suit: Suit::Cups,
            image_key: format!("decks/{id}/icons/cups.png"),
        }],
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
