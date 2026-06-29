//! Live smoke test against the real `agy` (Antigravity) CLI. Ignored by default; run with:
//!   cargo test -p providers --test agy_live -- --ignored

use providers::{AgyCliProvider, ImageProvider, ImageRequest, Size, TextProvider, TextRequest};

#[tokio::test]
#[ignore]
async fn agy_generates_text() {
    let p = AgyCliProvider::from_env();
    let res = p
        .complete(TextRequest {
            system: "You answer with a single word.".into(),
            prompt: "Reply with the word: pong".into(),
            seed: 0,
            params: Default::default(),
        })
        .await
        .expect("agy text call failed");
    assert!(!res.text.is_empty());
    eprintln!("agy text → {:?}", res.text);
}

#[tokio::test]
#[ignore]
async fn agy_generates_an_svg_rasterized_to_png() {
    let p = AgyCliProvider::from_env();
    let img = p
        .generate(ImageRequest {
            prompt: "a tarot 'wands' staff emblem, two colors, centered".into(),
            seed: 0,
            size: Size {
                width: 96,
                height: 96,
            },
            params: Default::default(),
        })
        .await
        .expect("agy image call failed");
    assert_eq!(img.mime, "image/png");
    assert_eq!(&img.bytes[..4], &[0x89, b'P', b'N', b'G']);
    eprintln!("agy image → {} png bytes", img.bytes.len());
}
