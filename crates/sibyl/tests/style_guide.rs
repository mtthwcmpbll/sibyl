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

async fn session_with_choice(forge: &Sibyl) -> String {
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
    out.session_id
}

#[tokio::test]
async fn style_guide_has_all_required_elements() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());
    let sid = session_with_choice(&forge).await;

    let guide = forge.generate_style_guide(&sid).await.unwrap();

    let store = FsStore::new(dir.path());
    assert!(
        store.exists(&guide.border_chrome_key),
        "border/chrome asset"
    );
    assert!(store.exists(&guide.card_back_key), "card back asset");
    assert!(
        !guide.shader_areas.is_empty(),
        "at least one shader/UV area"
    );
    assert!(!guide.flourishes.is_empty(), "flourishes present");
    assert!(!guide.prompt_used.trim().is_empty(), "prompt derived");
    assert_eq!(guide.version, 1);
    // Prompt was auto-derived from the chosen style (FR-008): owner did not restate it.
    assert!(guide.derived_from_style_option_id.starts_with("r0-opt"));
}

#[tokio::test]
async fn style_guide_requires_a_selected_style() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());
    let out = forge
        .generate_icon_styles(GenerateIconStyles {
            session_id: None,
            seed: Some(FIXED_SEED),
            style: "s".into(),
            count: 3,
        })
        .await
        .unwrap();
    // No selection made.
    let err = forge.generate_style_guide(&out.session_id).await;
    assert!(err.is_err(), "must require a selected style");
}
