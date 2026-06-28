use domain::model::{Personal, StyleOption, SuitIcon};
use domain::suit::Suit;

#[test]
fn personal_debug_is_redacted() {
    let secret = Personal::from("my mother's garden and the number 7");
    let rendered = format!("{secret:?}");
    assert!(
        !rendered.contains("garden"),
        "personal data leaked: {rendered}"
    );
    assert!(rendered.contains("redacted"));
}

#[test]
fn personal_empty_debug_is_marked_empty_not_redacted() {
    let empty = Personal::default();
    assert_eq!(format!("{empty:?}"), "Personal(<empty>)");
}

#[test]
fn style_option_requires_every_suit() {
    let full = StyleOption {
        id: "a".into(),
        label: "Option A".into(),
        suit_icons: Suit::STANDARD
            .iter()
            .map(|s| SuitIcon {
                suit: *s,
                image_key: format!("k/{s}"),
            })
            .collect(),
        prompt_used: "p".into(),
    };
    assert!(full.covers_all_suits(&Suit::STANDARD));

    let missing = StyleOption {
        suit_icons: vec![SuitIcon {
            suit: Suit::Cups,
            image_key: "k".into(),
        }],
        ..full.clone()
    };
    assert!(!missing.covers_all_suits(&Suit::STANDARD));
}

#[test]
fn suit_icon_round_trips_through_json() {
    let icon = SuitIcon {
        suit: Suit::Pentacles,
        image_key: "decks/x/icons/pentacles.png".into(),
    };
    let json = serde_json::to_string(&icon).unwrap();
    assert!(json.contains("pentacles"));
    let back: SuitIcon = serde_json::from_str(&json).unwrap();
    assert_eq!(icon, back);
}
