use domain::rng::sub_seed;
use domain::{CardStyleGuide, FlourishRef, FrontLayout, Rect, SessionStatus, ShaderArea};
use providers::{ImageRequest, Size};

use crate::error::{Result, SibylError};
use crate::prompt::build_image_prompt;
use crate::{rules, Sibyl};

impl Sibyl {
    /// Auto-derive a prompt from the chosen suit-icon style and generate the overall card
    /// style guide — border/chrome, card back, front layout, shader/UV areas, flourishes
    /// (FR-007/008). The owner does not restate the style.
    pub async fn generate_style_guide(&self, session_id: &str) -> Result<CardStyleGuide> {
        let mut session = self.load_session(session_id)?;
        let chosen = session
            .chosen_option()
            .ok_or(SibylError::NoStyleSelected)?
            .clone();

        let style = session.deck_style_text.as_str().to_string();
        let sg_seed = sub_seed(session.seed, "style-guide");

        // Each style-guide asset prepends ITS OWN authoritative rules (FR-019) — derived from
        // the chosen style automatically (FR-008), without restating it.
        let border_prompt = build_image_prompt(
            rules::CARD_BORDER,
            &style,
            "the card border and chrome frame",
        );
        let back_prompt = build_image_prompt(rules::CARD_BACK, &style, "the card back design");
        let flourish_prompt = build_image_prompt(
            rules::FLOURISH,
            &style,
            "a single decorative corner flourish",
        );

        let border_chrome_key = self
            .gen_asset(
                session_id,
                "style-guide/border-chrome.png",
                &border_prompt,
                sub_seed(sg_seed, "chrome"),
            )
            .await?;
        let card_back_key = self
            .gen_asset(
                session_id,
                "style-guide/card-back.png",
                &back_prompt,
                sub_seed(sg_seed, "back"),
            )
            .await?;
        let flourish_key = self
            .gen_asset(
                session_id,
                "style-guide/flourish-1.png",
                &flourish_prompt,
                sub_seed(sg_seed, "flourish"),
            )
            .await?;

        let guide = CardStyleGuide {
            id: format!("sg-{}", session.id),
            version: 1,
            derived_from_style_option_id: chosen.id.clone(),
            prompt_used: border_prompt.clone(),
            border_chrome_key,
            card_back_key,
            card_front_layout: FrontLayout {
                aspect: 0.66,
                suit_icon: Rect::new(0.05, 0.05, 0.18, 0.18),
                card_imagery: Rect::new(0.1, 0.18, 0.8, 0.62),
                title: Rect::new(0.0, 0.86, 1.0, 0.1),
            },
            shader_areas: vec![ShaderArea {
                id: "uv-frame".into(),
                rect: Rect::new(0.0, 0.0, 1.0, 1.0),
                kind: "foil".into(),
            }],
            flourishes: vec![FlourishRef {
                asset_key: flourish_key,
                rect: Rect::new(0.0, 0.0, 0.15, 0.15),
                rotation: 0.0,
            }],
        };

        session
            .provenance
            .prompts
            .insert("cardBorder".into(), border_prompt);
        session
            .provenance
            .prompts
            .insert("cardBack".into(), back_prompt);
        session
            .provenance
            .prompts
            .insert("flourish".into(), flourish_prompt);
        session.style_guide = Some(guide.clone());
        session.status = SessionStatus::StyleGuide;
        self.save_session(&session)?;
        Ok(guide)
    }

    /// Generate one image asset from a fully-composed prompt, store it, return the storage key.
    async fn gen_asset(
        &self,
        session_id: &str,
        suffix: &str,
        prompt: &str,
        seed: u64,
    ) -> Result<String> {
        let img = self
            .image
            .generate(ImageRequest {
                prompt: prompt.to_string(),
                seed,
                size: Size {
                    width: 64,
                    height: 64,
                },
                params: Default::default(),
            })
            .await?;
        let key = format!("decks/_drafts/{session_id}/{suffix}");
        self.store.put(&key, &img.bytes)?;
        Ok(key)
    }
}
