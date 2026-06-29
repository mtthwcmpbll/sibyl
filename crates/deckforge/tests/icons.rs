use deckforge::{DeckForge, GenerateIconStyles};
use domain::{Suit, FIXED_SEED};
use providers::FakeProvider;
use storage::{FsStore, Store};

fn forge(dir: &std::path::Path) -> DeckForge {
    DeckForge::new(
        Box::new(FakeProvider::new()),
        Box::new(FakeProvider::new()),
        Box::new(FsStore::new(dir)),
    )
}

fn input() -> GenerateIconStyles {
    GenerateIconStyles {
        session_id: None,
        seed: Some(FIXED_SEED),
        style: "midnight indigo and gold".into(),
        count: 3,
    }
}

#[tokio::test]
async fn generates_at_least_three_options_covering_all_suits() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());

    let out = forge.generate_icon_styles(input()).await.unwrap();
    assert!(
        out.style_options.len() >= 3,
        "SC-002: at least three options"
    );

    for opt in &out.style_options {
        assert!(
            opt.covers_all_suits(&Suit::STANDARD),
            "every option must cover all suits"
        );
        // Icons were actually stored.
        for icon in &opt.suit_icons {
            assert!(
                FsStore::new(dir.path()).exists(&icon.image_key),
                "icon not stored: {}",
                icon.image_key
            );
        }
    }
}

#[tokio::test]
async fn generation_is_deterministic_for_a_seed() {
    let dir1 = tempfile::tempdir().unwrap();
    let dir2 = tempfile::tempdir().unwrap();
    let a = forge(dir1.path())
        .generate_icon_styles(input())
        .await
        .unwrap();
    let b = forge(dir2.path())
        .generate_icon_styles(input())
        .await
        .unwrap();
    let a_prompts: Vec<_> = a.style_options.iter().map(|o| &o.prompt_used).collect();
    let b_prompts: Vec<_> = b.style_options.iter().map(|o| &o.prompt_used).collect();
    assert_eq!(a_prompts, b_prompts, "same seed → same options");
}

#[tokio::test]
async fn regenerate_preserves_inputs_but_changes_round() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());

    let first = forge.generate_icon_styles(input()).await.unwrap();
    let again = forge
        .generate_icon_styles(GenerateIconStyles {
            session_id: Some(first.session_id.clone()),
            ..input()
        })
        .await
        .unwrap();

    assert_eq!(first.session_id, again.session_id);
    // New round → different option ids (r0 vs r1).
    assert!(again.style_options[0].id.starts_with("r1-"));
    assert!(first.style_options[0].id.starts_with("r0-"));
}

#[tokio::test]
async fn select_icon_style_records_choice_and_rejects_unknown() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());
    let out = forge.generate_icon_styles(input()).await.unwrap();
    let chosen = out.style_options[1].id.clone();

    forge.select_icon_style(&out.session_id, &chosen).unwrap();
    let session = forge.load_session(&out.session_id).unwrap();
    assert_eq!(
        session.chosen_style_option_id.as_deref(),
        Some(chosen.as_str())
    );

    assert!(forge.select_icon_style(&out.session_id, "nope").is_err());
}
