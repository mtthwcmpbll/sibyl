use providers::{GenParams, TextProvider, TextRequest};

use crate::error::Result;

/// Compose the image-generation prompt from the **authoritative iconography rules** and the
/// **subordinate deck-style direction** (FR-019). Image providers expose only a single
/// prompt, so the rules/style precedence is enforced here, at composition time, and the
/// composed string is what gets recorded in provenance.
pub async fn compose_image_prompt(
    text: &dyn TextProvider,
    rules: &str,
    style: &str,
    context: &str,
    seed: u64,
) -> Result<String> {
    let system = if rules.trim().is_empty() {
        "AUTHORITATIVE ICONOGRAPHY RULES (override all style direction): \
         follow tasteful, coherent tarot iconography conventions."
            .to_string()
    } else {
        format!(
            "AUTHORITATIVE ICONOGRAPHY RULES (override all style direction): {}",
            rules.trim()
        )
    };

    let style_part = if style.trim().is_empty() {
        "(no explicit style given; choose something evocative within the rules)".to_string()
    } else {
        style.trim().to_string()
    };

    let prompt =
        format!("Style direction (subordinate to the rules): {style_part}. Context: {context}.");

    let resp = text
        .complete(TextRequest {
            system,
            prompt,
            seed,
            params: GenParams::default(),
        })
        .await?;

    Ok(resp.text)
}
