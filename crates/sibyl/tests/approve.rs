use domain::FIXED_SEED;
use providers::FakeProvider;
use sibyl::{GenerateIconStyles, Sibyl};
use storage::{FsStore, Store};

fn forge(dir: &std::path::Path) -> Sibyl {
    Sibyl::new(
        Box::new(FakeProvider::new()),
        Box::new(FakeProvider::new()),
        Box::new(FsStore::new(dir)),
    )
}

async fn drive_to_sample(forge: &Sibyl, seed: u64) -> String {
    let out = forge
        .generate_icon_styles(GenerateIconStyles {
            session_id: None,
            seed: Some(seed),
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
    out.session_id
}

#[tokio::test]
async fn approve_promotes_bundle_sets_active_and_records_provenance() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());
    let sid = drive_to_sample(&forge, FIXED_SEED).await;

    let deck = forge.approve_deck(&sid).unwrap();
    assert!(deck.active);

    let store = FsStore::new(dir.path());
    // Assets were promoted out of the draft and all referenced keys resolve.
    for icon in &deck.suit_icons {
        assert!(icon.image_key.starts_with("decks/deck-"), "relocated key");
        assert!(store.exists(&icon.image_key), "icon present in bundle");
    }
    assert!(store.exists(&deck.style_guide.border_chrome_key));
    assert!(store.exists(&deck.style_guide.card_back_key));

    // Draft discarded; active deck loads back identically (SC-003/004).
    assert!(!store.exists(&format!("decks/_drafts/{sid}/session.json")));
    let active = forge.get_active_deck().unwrap().unwrap();
    assert_eq!(active.id, deck.id);

    // Provenance is sufficient to explain/regenerate (FR-014/T072).
    assert_eq!(active.provenance.seed, FIXED_SEED);
    assert!(active.provenance.chosen_style_option_id.is_some());
    assert!(active.provenance.court_card_shown.is_some());
    assert!(active.provenance.prompts.contains_key("cardBorder"));
    assert!(active.provenance.prompts.contains_key("cardBack"));
    assert!(active.provenance.prompts.contains_key("flourish"));
    assert!(active.provenance.prompts.contains_key("sampleImagery"));
}

#[tokio::test]
async fn reject_leaves_no_active_deck_and_returns_inputs() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());
    let sid = drive_to_sample(&forge, FIXED_SEED).await;

    let inputs = forge.reject_and_restart(&sid).unwrap();
    assert_eq!(inputs.style, "midnight indigo and gold");
    assert!(
        forge.get_active_deck().unwrap().is_none(),
        "no active deck after reject"
    );
    let store = FsStore::new(dir.path());
    assert!(!store.exists(&format!("decks/_drafts/{sid}/session.json")));
}

#[tokio::test]
async fn second_approval_reassigns_the_single_active_deck() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());

    let s1 = drive_to_sample(&forge, 111).await;
    let d1 = forge.approve_deck(&s1).unwrap();
    let s2 = drive_to_sample(&forge, 222).await;
    let d2 = forge.approve_deck(&s2).unwrap();

    assert_ne!(d1.id, d2.id);
    assert_eq!(forge.get_active_deck().unwrap().unwrap().id, d2.id);
}
