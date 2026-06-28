use deckforge::compose_image_prompt;
use providers::FakeProvider;

#[tokio::test]
async fn rules_are_authoritative_over_style() {
    let text = FakeProvider::new();
    let composed = compose_image_prompt(
        &text,
        "only line art, no color, sacred geometry",
        "warm watercolor with lots of color",
        "suit-set option 0",
        7,
    )
    .await
    .unwrap();

    // The authoritative rules framing must be present and precede the style direction.
    assert!(composed.contains("AUTHORITATIVE"));
    let rules_pos = composed
        .find("sacred geometry")
        .expect("rules text present");
    let style_pos = composed
        .find("warm watercolor")
        .expect("style text present");
    assert!(
        rules_pos < style_pos,
        "rules must appear before (outrank) style: {composed}"
    );
}

#[tokio::test]
async fn empty_rules_still_produce_authoritative_framing() {
    let text = FakeProvider::new();
    let composed = compose_image_prompt(&text, "", "dreamy pastels", "ctx", 1)
        .await
        .unwrap();
    assert!(composed.contains("AUTHORITATIVE"));
    assert!(composed.contains("dreamy pastels"));
}
