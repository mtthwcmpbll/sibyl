use providers::{FakeProvider, ImageProvider, ImageRequest, Size, TextProvider, TextRequest};

fn img_req(prompt: &str, seed: u64) -> ImageRequest {
    ImageRequest {
        prompt: prompt.to_string(),
        seed,
        size: Size::default(),
        params: Default::default(),
    }
}

#[tokio::test]
async fn image_output_is_deterministic_for_same_prompt_and_seed() {
    let p = FakeProvider::new();
    let a = p.generate(img_req("queen of cups", 42)).await.unwrap();
    let b = p.generate(img_req("queen of cups", 42)).await.unwrap();
    assert_eq!(a, b, "fake image must be deterministic");
    assert_eq!(a.mime, "image/png");
    assert!(!a.bytes.is_empty());
}

#[tokio::test]
async fn image_output_varies_with_prompt() {
    let p = FakeProvider::new();
    let a = p.generate(img_req("queen of cups", 42)).await.unwrap();
    let b = p.generate(img_req("king of swords", 42)).await.unwrap();
    assert_ne!(a.bytes, b.bytes, "different prompts should differ");
}

#[tokio::test]
async fn text_preserves_system_then_prompt() {
    let p = FakeProvider::new();
    let out = p
        .complete(TextRequest {
            system: "RULES".into(),
            prompt: "style".into(),
            seed: 1,
            params: Default::default(),
        })
        .await
        .unwrap();
    assert!(out.text.starts_with("RULES"));
}

#[tokio::test]
async fn provider_ids_are_stable() {
    let p = FakeProvider::new();
    assert_eq!(TextProvider::id(&p).provider, "fake");
    assert_eq!(ImageProvider::id(&p).provider, "fake");
}
