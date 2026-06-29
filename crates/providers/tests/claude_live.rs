//! Live smoke test against the real `claude` CLI. Ignored by default (consumes the user's
//! subscription); run explicitly:  cargo test -p providers --test claude_live -- --ignored

use providers::{ClaudeCliProvider, ImageProvider, ImageRequest, Size, TextProvider, TextRequest};

#[tokio::test]
#[ignore]
async fn claude_generates_text() {
    let p = ClaudeCliProvider::from_env();
    let res = p
        .complete(TextRequest {
            system: "You answer with a single word.".into(),
            prompt: "Reply with the word: pong".into(),
            seed: 0,
            params: Default::default(),
        })
        .await
        .expect("claude text call failed");
    assert!(!res.text.is_empty());
    eprintln!("claude text → {:?}", res.text);
}

#[tokio::test]
#[ignore]
async fn claude_generates_an_svg_rasterized_to_png() {
    let p = ClaudeCliProvider::from_env();
    let img = p
        .generate(ImageRequest {
            prompt: "a tarot 'cups' chalice emblem, two colors, centered".into(),
            seed: 0,
            size: Size {
                width: 96,
                height: 96,
            },
            params: Default::default(),
        })
        .await
        .expect("claude image call failed");
    assert_eq!(img.mime, "image/png");
    assert_eq!(&img.bytes[..4], &[0x89, b'P', b'N', b'G']);
    eprintln!("claude image → {} png bytes", img.bytes.len());
}
