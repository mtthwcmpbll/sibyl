//! T063 — end-to-end pipeline on the fake provider + fixed seed: inputs → icons → style
//! guide → sample → approve yields a single active, self-contained bundle (Principles III/V).

use domain::FIXED_SEED;
use providers::FakeProvider;
use sibyl::{GenerateIconStyles, Sibyl};
use storage::{FsStore, Store};

#[tokio::test]
async fn full_pipeline_produces_an_active_self_contained_bundle() {
    let dir = tempfile::tempdir().unwrap();
    let forge = Sibyl::new(
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
    // Each artifact's LLM call was given ITS OWN rules file (distinctive phrases prove the
    // right rules reached the right call), with the owner's style prepended-after.
    let p = &deck.provenance.prompts;
    assert!(out.style_options[0].prompt_used.contains("four suit icons")); // suit_icons.md
    assert!(p["cardBorder"].contains("rectangular frame")); // card_border.md
    assert!(p["cardBack"].contains("identical for every card")); // card_back.md
    assert!(p["flourish"].contains("small ornament")); // flourish.md
    assert!(p["sampleImagery"].contains("evocative illustration")); // background_image.md
                                                                    // …and every one carries the owner's deck style, subordinate to the rules.
    for key in ["cardBorder", "cardBack", "flourish", "sampleImagery"] {
        assert!(
            p[key].contains("midnight indigo and gold"),
            "style missing from {key}"
        );
        assert!(
            p[key].contains("AUTHORITATIVE"),
            "rules framing missing from {key}"
        );
    }

    // The draft is gone and the active deck loads back identically (SC-003/004).
    assert!(!store.exists(&format!("decks/_drafts/{}/session.json", out.session_id)));
    assert_eq!(forge.get_active_deck().unwrap().unwrap().id, deck.id);
}
