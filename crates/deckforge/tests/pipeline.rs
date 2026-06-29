//! T063 — end-to-end pipeline on the fake provider + fixed seed: inputs → icons → style
//! guide → sample → approve yields a single active, self-contained bundle (Principles III/V).

use deckforge::{DeckForge, GenerateIconStyles};
use domain::FIXED_SEED;
use providers::FakeProvider;
use storage::{FsStore, Store};

#[tokio::test]
async fn full_pipeline_produces_an_active_self_contained_bundle() {
    let dir = tempfile::tempdir().unwrap();
    let forge = DeckForge::new(
        Box::new(FakeProvider::new()),
        Box::new(FakeProvider::new()),
        Box::new(FsStore::new(dir.path())),
    );

    let out = forge
        .generate_icon_styles(GenerateIconStyles {
            session_id: None,
            seed: Some(FIXED_SEED),
            style: "midnight indigo and gold".into(),
            count: 3,
        })
        .await
        .unwrap();
    forge
        .select_icon_style(&out.session_id, &out.style_options[0].id)
        .unwrap();
    forge.generate_style_guide(&out.session_id).await.unwrap();
    forge.compose_sample_card(&out.session_id).await.unwrap();
    let deck = forge.approve_deck(&out.session_id).unwrap();

    let store = FsStore::new(dir.path());

    // Self-contained: every key the manifest references resolves within the bundle.
    for icon in &deck.suit_icons {
        assert!(store.exists(&icon.image_key), "missing {}", icon.image_key);
    }
    assert!(store.exists(&deck.style_guide.border_chrome_key));
    assert!(store.exists(&deck.style_guide.card_back_key));
    for f in &deck.style_guide.flourishes {
        assert!(
            store.exists(&f.asset_key),
            "missing flourish {}",
            f.asset_key
        );
    }

    // Active, with provenance sufficient to explain/regenerate (FR-014).
    assert!(deck.active);
    assert_eq!(deck.provenance.seed, FIXED_SEED);
    assert!(deck.provenance.chosen_style_option_id.is_some());
    assert!(deck.provenance.court_card_shown.is_some());
    assert!(deck.provenance.prompts.contains_key("styleGuide"));
    assert!(deck.provenance.prompts.contains_key("sampleImagery"));

    // The draft is gone and the active deck loads back identically (SC-003/004).
    assert!(!store.exists(&format!("decks/_drafts/{}/session.json", out.session_id)));
    assert_eq!(forge.get_active_deck().unwrap().unwrap().id, deck.id);
}
