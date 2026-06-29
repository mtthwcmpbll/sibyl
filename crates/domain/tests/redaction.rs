//! T065 — personal data must never appear in Debug/log output (constitution Principle VI).
//! The owner's deck style is personal; the iconography rules are a non-personal app resource.

use domain::{DeckCreationSession, GenerationProvenance, Personal};

#[test]
fn session_debug_redacts_the_owner_style_but_not_the_app_rules() {
    let session = DeckCreationSession::new(
        "sess-1",
        7,
        "app rules: four suits, no human faces", // app resource — not personal
        Personal::from("the lake house in Vermont, summer 1998"),
    );
    let dbg = format!("{session:?}");
    assert!(!dbg.contains("Vermont"), "personal style leaked: {dbg}");
    assert!(dbg.contains("redacted"));
}

#[test]
fn provenance_debug_redacts_personal_style() {
    let prov = GenerationProvenance {
        seed: 1,
        iconography_rules: "app rules text".into(), // not personal
        deck_style_text: Personal::from("secret personal style"),
        ..Default::default()
    };
    let dbg = format!("{prov:?}");
    assert!(
        !dbg.contains("secret"),
        "provenance leaked personal data: {dbg}"
    );
}
