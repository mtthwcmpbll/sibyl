//! Build the prompt sent to the image provider for a generated artifact: the artifact's
//! authoritative, app-defined rules first, then the owner's deck style as subordinate
//! direction, then the specific subject. The rules are PREPENDED and take precedence
//! (FR-019); the composed prompt is what gets recorded in provenance.

/// Compose an image-generation prompt by prepending the artifact `rules` to the owner's
/// `style` and the specific `subject`. Pure (no LLM call) — there is exactly one LLM call per
/// artifact: the image generation itself.
pub fn build_image_prompt(rules: &str, style: &str, subject: &str) -> String {
    let rules = rules.trim();
    let style = style.trim();
    let style_line = if style.is_empty() {
        "(no explicit style given; choose something evocative within the rules)"
    } else {
        style
    };

    format!(
        "AUTHORITATIVE RULES (override all style direction):\n{rules}\n\n\
         Deck style (subordinate to the rules): {style_line}\n\n\
         Subject: {subject}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_are_prepended_and_authoritative_over_style() {
        let p = build_image_prompt(
            "only line art, no color, sacred geometry",
            "warm watercolor with lots of color",
            "the cups suit emblem",
        );
        assert!(p.contains("AUTHORITATIVE"));
        let rules_pos = p.find("sacred geometry").expect("rules present");
        let style_pos = p.find("warm watercolor").expect("style present");
        assert!(
            rules_pos < style_pos,
            "rules must precede (outrank) style:\n{p}"
        );
        assert!(p.contains("the cups suit emblem"));
    }

    #[test]
    fn empty_style_still_keeps_rules() {
        let p = build_image_prompt("four suits only", "", "a card back");
        assert!(p.contains("AUTHORITATIVE"));
        assert!(p.contains("four suits only"));
        assert!(p.contains("a card back"));
    }
}
