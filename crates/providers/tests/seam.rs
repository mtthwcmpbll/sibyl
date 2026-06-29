//! T064 — provider-seam guard (constitution Principle I). No vendor SDK/type may appear
//! outside an adapter module. Real HTTP adapters belong in `http.rs` (the only allowed place
//! for vendor crate references); everything else must stay vendor-neutral.

use std::fs;

#[test]
fn no_vendor_tokens_leak_outside_adapter_modules() {
    // Tokens that would signal a concrete vendor leaking into neutral code.
    let banned = [
        "reqwest",
        "openai",
        "anthropic",
        "genai",
        "google_generativeai",
        "cohere",
        "ollama",
        "mistralai",
    ];
    // Modules permitted to reference a vendor SDK or tool (the adapter boundary).
    let allowed = ["http.rs", "claude.rs", "agy.rs"];

    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    for entry in fs::read_dir(src).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if allowed.contains(&name.as_str()) {
            continue;
        }
        let content = fs::read_to_string(&path).unwrap();
        for token in &banned {
            assert!(
                !content.contains(token),
                "vendor token '{token}' leaked into {name} — keep it inside an adapter (Principle I)"
            );
        }
    }
}
