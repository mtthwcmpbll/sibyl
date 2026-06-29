//! Headless verification of the command layer (the Tauri shell wraps these 1:1). Covers the
//! full happy path, asset encoding, and the error→code mapping from contracts/ipc-commands.md.

use app_core as app;
use providers::FakeProvider;
use sibyl::Sibyl;
use storage::FsStore;

fn forge(dir: &std::path::Path) -> Sibyl {
    Sibyl::new(
        Box::new(FakeProvider::new()),
        Box::new(FakeProvider::new()),
        Box::new(FsStore::new(dir)),
    )
}

#[tokio::test]
async fn full_command_flow_produces_active_deck_and_decodable_assets() {
    let dir = tempfile::tempdir().unwrap();
    let f = forge(dir.path());

    let out = app::generate_icon_styles(&f, None, "indigo and gold".into(), Some(3))
        .await
        .unwrap();
    assert!(out.style_options.len() >= 3);

    // get_asset returns valid, decodable base64 for a real icon key.
    let key = out.style_options[0].suit_icons[0].image_key.clone();
    let asset = app::get_asset(&f, &key).unwrap();
    assert_eq!(asset.mime, "image/png");
    use base64::Engine;
    assert!(base64::engine::general_purpose::STANDARD
        .decode(&asset.base64)
        .is_ok());

    app::select_icon_style(&f, &out.session_id, &out.style_options[0].id).unwrap();
    app::generate_style_guide(&f, &out.session_id)
        .await
        .unwrap();
    app::compose_sample_card(&f, &out.session_id).await.unwrap();
    let deck = app::approve_deck(&f, &out.session_id).unwrap();
    assert!(deck.active);
    assert_eq!(app::get_active_deck(&f).unwrap().unwrap().id, deck.id);
}

#[tokio::test]
async fn errors_map_to_contract_codes() {
    let dir = tempfile::tempdir().unwrap();
    let f = forge(dir.path());

    let e = app::generate_style_guide(&f, "nope").await.unwrap_err();
    assert_eq!(e.code, "unknown_session");
    assert!(!e.retryable);

    let out = app::generate_icon_styles(&f, None, "s".into(), None)
        .await
        .unwrap();
    let e = app::select_icon_style(&f, &out.session_id, "bad").unwrap_err();
    assert_eq!(e.code, "unknown_option");
    let e = app::generate_style_guide(&f, &out.session_id)
        .await
        .unwrap_err();
    assert_eq!(e.code, "no_style_selected");
}

#[test]
fn app_data_dir_honors_sibyl_home() {
    std::env::set_var("SIBYL_HOME", "/tmp/sibyl-test-home");
    assert_eq!(
        app::app_data_dir(),
        std::path::PathBuf::from("/tmp/sibyl-test-home")
    );
    std::env::remove_var("SIBYL_HOME");
}
