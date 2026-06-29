use deckforge::{DeckForge, GenerateIconStyles};
use domain::court::CourtCardId;
use domain::FIXED_SEED;
use providers::FakeProvider;
use storage::{FsStore, Store};

fn forge(dir: &std::path::Path) -> DeckForge {
    DeckForge::new(
        Box::new(FakeProvider::new()),
        Box::new(FakeProvider::new()),
        Box::new(FsStore::new(dir)),
    )
}

async fn drive_to_sample(forge: &DeckForge, seed: u64) -> (String, domain::SampleCard) {
    let out = forge
        .generate_icon_styles(GenerateIconStyles {
            session_id: None,
            seed: Some(seed),
            rules: "bold geometric glyphs".into(),
            style: "midnight indigo and gold".into(),
            count: 3,
        })
        .await
        .unwrap();
    forge
        .select_icon_style(&out.session_id, &out.style_options[0].id)
        .unwrap();
    forge.generate_style_guide(&out.session_id).await.unwrap();
    let sample = forge.compose_sample_card(&out.session_id).await.unwrap();
    (out.session_id, sample)
}

#[tokio::test]
async fn sample_is_always_a_court_card_with_stored_front_and_back() {
    let dir = tempfile::tempdir().unwrap();
    let forge = forge(dir.path());
    let (_sid, sample) = drive_to_sample(&forge, FIXED_SEED).await;

    // Always one of the 16 court cards (clarification 2026-06-28).
    assert!(CourtCardId::all().contains(&sample.court_card));

    let store = FsStore::new(dir.path());
    assert!(store.exists(&sample.front_key), "front composited & stored");
    assert!(store.exists(&sample.back_key), "back present");
    assert_eq!(sample.approval, domain::Approval::Pending);
}

#[tokio::test]
async fn sample_front_is_byte_stable_for_a_seed() {
    let dir1 = tempfile::tempdir().unwrap();
    let dir2 = tempfile::tempdir().unwrap();
    let f1 = forge(dir1.path());
    let f2 = forge(dir2.path());

    let (_s1, a) = drive_to_sample(&f1, FIXED_SEED).await;
    let (_s2, b) = drive_to_sample(&f2, FIXED_SEED).await;
    assert_eq!(a.court_card, b.court_card, "seed → same court card");

    let front_a = FsStore::new(dir1.path()).get(&a.front_key).unwrap();
    let front_b = FsStore::new(dir2.path()).get(&b.front_key).unwrap();
    assert_eq!(
        front_a, front_b,
        "canonical front render is reproducible (SC-007)"
    );
}
