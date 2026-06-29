//! T065 — personal data must never appear in Debug/log output (constitution Principle VI).

use domain::{DeckCreationSession, GenerationProvenance, Personal};

#[test]
fn session_debug_does_not_leak_personal_inputs() {
    let session = DeckCreationSession::new(
        "sess-1",
        7,
        Personal::from("my mother's maiden name is Aldridge"),
        Personal::from("the lake house in Vermont, summer 1998"),
    );
    let dbg = format!("{session:?}");
    assert!(!dbg.contains("Aldridge"), "rules leaked: {dbg}");
    assert!(!dbg.contains("Vermont"), "style leaked: {dbg}");
    assert!(dbg.contains("redacted"));
}

#[test]
fn provenance_debug_redacts_personal_fields() {
    let prov = GenerationProvenance {
        seed: 1,
        iconography_rules: Personal::from("secret iconography rules"),
        deck_style_text: Personal::from("secret personal style"),
        ..Default::default()
    };
    let dbg = format!("{prov:?}");
    assert!(
        !dbg.contains("secret"),
        "provenance leaked personal data: {dbg}"
    );
}
